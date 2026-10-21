from __future__ import annotations

from collections.abc import Sequence


import polars as pl
from fairspec_metadata import ColumnType
from fairspec_metadata import CellError
from fairspec_metadata import ColumnTypeError
from fairspec_metadata import TableError
from pydantic import TypeAdapter

from fairspec_table.models import CellMapping, ColumnMapping
from fairspec_table.models.table import Table
from fairspec_table.settings import QUERY_ENGINE, NUMBER_COLUMN_NAME

from .checks.const import check_cell_const
from .checks.enum import check_cell_enum
from .checks.max_items import check_cell_max_items
from .checks.max_length import check_cell_max_length
from .checks.maximum import create_check_cell_maximum
from .checks.min_items import check_cell_min_items
from .checks.min_length import check_cell_min_length
from .checks.minimum import create_check_cell_minimum
from .checks.missing import check_cell_missing
from .checks.multiple_of import check_cell_multiple_of
from .checks.pattern import check_cell_pattern
from .checks.type import check_cell_type
from .normalize import normalize_column
from .types.array import inspect_array_column
from .types.duration import inspect_duration_column
from .types.geojson import inspect_geojson_column
from .types.object import inspect_object_column
from .types.topojson import inspect_topojson_column
from .types.wkb import inspect_wkb_column
from .types.wkt import inspect_wkt_column


def inspect_columns(
    mappings: list[ColumnMapping],
    table: Table,
    *,
    max_errors: int,
    flag_exprs: Sequence[pl.Expr] = (),
) -> dict[str, list[TableError]]:
    """Check every column against its schema, keyed by column name in mapping order.

    Each of `flag_exprs`, an always-false Boolean, is evaluated in the same read.
    """
    errors: dict[str, list[TableError]] = {}
    polars_mappings: list[ColumnMapping] = []

    for mapping in mappings:
        errors[mapping.target.name] = _inspect_type(mapping)
        is_checked = not errors[mapping.target.name]
        if is_checked and mapping.target.type not in ROW_INSPECTED_TYPES:
            polars_mappings.append(mapping)

    if polars_mappings or flag_exprs:
        errors.update(
            _inspect_cells_in_polars(
                polars_mappings, table, max_errors=max_errors, flag_exprs=flag_exprs
            )
        )

    error_count = 0
    for mapping in mappings:
        column_errors = errors[mapping.target.name]
        is_checked = not column_errors and error_count < max_errors
        if is_checked and mapping.target.type in ROW_INSPECTED_TYPES:
            column_errors.extend(_inspect_cells_by_row(mapping, table))
        error_count += len(column_errors)

    return errors


COMPAT_MAPPING: dict[type[pl.DataType], list[str]] = {
    pl.Boolean: ["boolean"],
    pl.Categorical: ["string"],
    pl.Date: ["date"],
    pl.Float32: ["number", "integer"],
    pl.Float64: ["number", "integer"],
    pl.Int8: ["integer", "number"],
    pl.Int16: ["integer", "number"],
    pl.Int32: ["integer", "number"],
    pl.Int64: ["integer", "number"],
    pl.List: ["list"],
    pl.String: ["unknown"],
    pl.Time: ["time", "local-time"],
    pl.UInt8: ["integer", "number"],
    pl.UInt16: ["integer", "number"],
    pl.UInt32: ["integer", "number"],
    pl.UInt64: ["integer", "number"],
}


def _inspect_type(mapping: ColumnMapping) -> list[TableError]:
    errors: list[TableError] = []

    compat_types = _get_compat_types(mapping)

    target_types = {mapping.target.type, "unknown"}
    is_compat = bool(set(compat_types) & target_types)

    if not is_compat:
        errors.append(
            ColumnTypeError(
                type="column/type",
                columnName=mapping.target.name,
                expectedColumnType=ColumnType(mapping.target.type),
                actualColumnType=ColumnType(
                    compat_types[0] if compat_types else "unknown"
                ),
            )
        )

    return errors


def _get_compat_types(mapping: ColumnMapping) -> list[str]:
    if issubclass(mapping.source.type, pl.Datetime):
        return ["date-time" if mapping.source.time_zone else "local-date-time"]
    for dtype_cls, types in COMPAT_MAPPING.items():
        if issubclass(mapping.source.type, dtype_cls):
            return types
    return []


# These types are parsed row by row outside polars, so each keeps its own scan
ROW_INSPECTED_TYPES = frozenset(
    {"duration", "wkb", "wkt", "array", "object", "geojson", "topojson"}
)


def _inspect_cells_by_row(mapping: ColumnMapping, table: Table) -> list[TableError]:
    target = mapping.target
    match target.type:
        case "duration":
            return inspect_duration_column(target, table, mapping)  # type: ignore[arg-type]
        case "wkb":
            return inspect_wkb_column(target, table, mapping)  # type: ignore[arg-type]
        case "wkt":
            return inspect_wkt_column(target, table, mapping)  # type: ignore[arg-type]
        case "array":
            return inspect_array_column(target, table, mapping)  # type: ignore[arg-type]
        case "object":
            return inspect_object_column(target, table, mapping)  # type: ignore[arg-type]
        case "geojson":
            return inspect_geojson_column(target, table, mapping)  # type: ignore[arg-type]
        case "topojson":
            return inspect_topojson_column(target, table, mapping)  # type: ignore[arg-type]
        case _:
            return []


CHECK_FUNCTIONS = [
    check_cell_type,
    check_cell_missing,
    check_cell_enum,
    check_cell_const,
    create_check_cell_minimum(),
    create_check_cell_maximum(),
    create_check_cell_minimum(is_exclusive=True),
    create_check_cell_maximum(is_exclusive=True),
    check_cell_multiple_of,
    check_cell_min_length,
    check_cell_max_length,
    check_cell_min_items,
    check_cell_max_items,
    check_cell_pattern,
]


def _inspect_cells_in_polars(
    mappings: list[ColumnMapping],
    table: Table,
    *,
    max_errors: int,
    flag_exprs: Sequence[pl.Expr],
) -> dict[str, list[TableError]]:
    errors: dict[str, list[TableError]] = {m.target.name: [] for m in mappings}
    flags = [expr.alias(_get_flag_name(index)) for index, expr in enumerate(flag_exprs)]
    is_flagged = [pl.col(_get_flag_name(index)) for index in range(len(flags))]

    # The error columns carry a tag, not the error itself: a literal is broadcast to
    # every row, so a JSON template would materialize a full-length string column per
    # check. Templates are resolved back in Python once the frame is filtered down.
    checked: list[tuple[int, ColumnMapping, list[CellError], pl.Expr]] = []
    for index, mapping in enumerate(mappings):
        cell_mapping = CellMapping(
            source=pl.col(_get_source_name(index)),
            target=pl.col(_get_target_name(index)),
        )

        checks = [
            check
            for check_cell in CHECK_FUNCTIONS
            if (check := check_cell(mapping.target, cell_mapping))
        ]
        if not checks:
            continue

        # The first failing check wins, so the chain is built from the last one back
        error_expr = pl.lit(None, dtype=pl.UInt8)
        for error_tag in reversed(range(len(checks))):
            error_expr = (
                pl.when(checks[error_tag].is_error_expr)
                .then(pl.lit(error_tag, dtype=pl.UInt8))
                .otherwise(error_expr)
            )

        templates = [check.error_template for check in checks]
        checked.append((index, mapping, templates, error_expr))

    if not checked:
        if flags:
            table.select(flags).filter(pl.any_horizontal(is_flagged)).collect(
                engine=QUERY_ENGINE
            )
        return errors

    check_table = (
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .select(
            pl.col(NUMBER_COLUMN_NAME),
            *flags,
            *(
                expr
                for index, mapping, _, _ in checked
                for expr in (
                    normalize_column(mapping).alias(_get_target_name(index)),
                    normalize_column(mapping, keep_type=True).alias(
                        _get_source_name(index)
                    ),
                    pl.col(mapping.source.name).alias(_get_cell_name(index)),
                )
            ),
        )
        .with_columns(
            error_expr.alias(_get_error_name(index))
            for index, _, _, error_expr in checked
        )
    )

    # A lazy frame caches nothing, so every column shares this one collect: a collect
    # per column would parse the whole source once for each. The flags join the filter
    # only so the optimizer keeps them; they are never true
    check_frame: pl.DataFrame = (  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        check_table.filter(
            pl.any_horizontal(
                *(
                    pl.col(_get_error_name(index)).is_not_null()
                    for index, _, _, _ in checked
                ),
                *is_flagged,
            )
        )
        .select(
            pl.col(NUMBER_COLUMN_NAME),
            *(
                pl.col(_get_cell_name(index), _get_error_name(index))
                for index, _, _, _ in checked
            ),
        )
        .head(max_errors)
        .collect(engine=QUERY_ENGINE)
    )

    _cell_error_adapter = TypeAdapter(CellError)
    row_numbers = check_frame.get_column(NUMBER_COLUMN_NAME).to_list()
    for index, mapping, templates, _ in checked:
        cells = check_frame.get_column(_get_cell_name(index)).to_list()
        error_tags = check_frame.get_column(_get_error_name(index)).to_list()

        for row_number, cell, error_tag in zip(row_numbers, cells, error_tags):
            if error_tag is None:
                continue

            error_dict = templates[error_tag].model_dump(by_alias=True)
            error_dict["rowNumber"] = row_number
            error_dict["cell"] = str(cell if cell is not None else "")
            errors[mapping.target.name].append(
                _cell_error_adapter.validate_python(error_dict)
            )

    return errors


def _get_target_name(index: int) -> str:
    return f"fairspec:target:{index}"


def _get_source_name(index: int) -> str:
    return f"fairspec:source:{index}"


def _get_cell_name(index: int) -> str:
    return f"fairspec:cell:{index}"


def _get_flag_name(index: int) -> str:
    return f"fairspec:flag:{index}"


def _get_error_name(index: int) -> str:
    return f"fairspec:error:{index}"
