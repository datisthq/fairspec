from __future__ import annotations

from collections.abc import Callable, Sequence


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

FlagFactory = Callable[[Callable[[str], pl.Expr | None]], pl.Expr]


def inspect_columns(
    mappings: list[ColumnMapping],
    table: Table,
    *,
    max_errors: int,
    flags: Sequence[FlagFactory] = (),
    plain_integer_columns: frozenset[str] = frozenset(),
) -> dict[str, list[TableError]]:
    """Check every column against its schema, keyed by column name in mapping order.

    Each of `flags` builds an always-false Boolean, evaluated in the same read, from a
    reader of the typed value of a column this read parses (None for any other). The
    integer columns in `plain_integer_columns` read only the plain form: a cell with a
    fraction or an exponent fails their type check.
    """
    errors: dict[str, list[TableError]] = {}
    polars_mappings: list[ColumnMapping] = []

    for mapping in mappings:
        errors[mapping.target.name] = _inspect_type(mapping)
        is_checked = not errors[mapping.target.name]
        if is_checked and mapping.target.type not in ROW_INSPECTED_TYPES:
            polars_mappings.append(mapping)

    if polars_mappings or flags:
        errors.update(
            _inspect_cells_in_polars(
                polars_mappings,
                table,
                max_errors=max_errors,
                flags=flags,
                plain_integer_columns=plain_integer_columns,
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
    pl.List: ["list", "array"],
    pl.String: ["unknown"],
    pl.Struct: ["object", "geojson", "topojson"],
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
    check_cell_pattern,
    check_cell_min_items,
    check_cell_max_items,
]

# A cell failing one of these has that error alone; its constraints go unchecked
SHORT_CIRCUIT_CHECKS = (check_cell_type, check_cell_missing)


def _inspect_cells_in_polars(
    mappings: list[ColumnMapping],
    table: Table,
    *,
    max_errors: int,
    flags: Sequence[FlagFactory],
    plain_integer_columns: frozenset[str],
) -> dict[str, list[TableError]]:
    errors: dict[str, list[TableError]] = {m.target.name: [] for m in mappings}
    is_flagged = [pl.col(_get_flag_name(index)) for index in range(len(flags))]

    # The error columns carry a tag, not the error itself: a literal is broadcast to
    # every row, so a JSON template would materialize a full-length string column per
    # check. Templates are resolved back in Python once the frame is filtered down.
    candidates = [
        (index, mapping, _create_check_pairs(mapping, index), _get_is_parsed(mapping))
        for index, mapping in enumerate(mappings)
        if mapping.target.type != "unknown"
    ]

    # A target read as its source is null only where the source is, so it never fails the
    # type check; a column left with no check is still read (carried), as the read
    # reports a long row or invalid UTF-8 only in the columns it reads
    has_check = any(is_parsed or len(pairs) > 1 for _, _, pairs, is_parsed in candidates)
    checked: list[tuple[int, ColumnMapping, list[CellError], pl.Expr]] = []
    carried: list[tuple[int, ColumnMapping]] = []
    for index, mapping, all_pairs, is_parsed in candidates:
        pairs = [
            (check_cell, check)
            for check_cell, check in all_pairs
            if is_parsed or not has_check or check_cell is not check_cell_type
        ]
        if not pairs:
            carried.append((index, mapping))
            continue

        # The error column is a bitmask over the checks, bit n set when check n fails, and
        # 0 when none does: nulling the 0 would copy the whole mask, as the streaming
        # engine evaluates an expression once per use
        bits = [pl.lit(1 << tag, dtype=pl.UInt32) for tag in range(len(pairs))]
        zero = pl.lit(0, dtype=pl.UInt32)
        constraint_exprs = [
            pl.when(check.is_error_expr).then(bit).otherwise(zero)
            for (check_cell, check), bit in zip(pairs, bits)
            if check_cell not in SHORT_CIRCUIT_CHECKS
        ]
        error_expr = pl.sum_horizontal(constraint_exprs) if constraint_exprs else zero
        for (check_cell, check), bit in reversed(list(zip(pairs, bits))):
            if check_cell in SHORT_CIRCUIT_CHECKS:
                error_expr = pl.when(check.is_error_expr).then(bit).otherwise(error_expr)

        templates = [check.error_template for _, check in pairs]
        checked.append((index, mapping, templates, error_expr))

    if not checked:
        if flags:
            flag_exprs = _create_flag_exprs(flags, {})
            table.select(flag_exprs).filter(pl.any_horizontal(is_flagged)).collect(
                engine=QUERY_ENGINE
            )
        return errors

    # The targets come first so the flags read them: the streaming engine evaluates a
    # repeated expression once per use, so a flag parsing a column again parses it twice
    target_names = {
        mapping.source.name: _get_target_name(index) for index, mapping, _, _ in checked
    }
    check_table = (
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .with_columns(
            normalize_column(
                mapping, is_plain=mapping.source.name in plain_integer_columns
            ).alias(_get_target_name(index))
            for index, mapping, _, _ in checked
        )
        .select(
            pl.col(NUMBER_COLUMN_NAME),
            *_create_flag_exprs(flags, target_names),
            *(
                expr
                for index, mapping, _, _ in checked
                for expr in (
                    pl.col(_get_target_name(index)),
                    normalize_column(mapping, keep_type=True).alias(
                        _get_source_name(index)
                    ),
                    pl.col(mapping.source.name).alias(_get_cell_name(index)),
                )
            ),
            *(
                pl.col(mapping.source.name).alias(_get_cell_name(index))
                for index, mapping in carried
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
                *(pl.col(_get_error_name(index)) != 0 for index, _, _, _ in checked),
                *is_flagged,
            )
        )
        .select(
            pl.col(NUMBER_COLUMN_NAME),
            *(
                pl.col(_get_cell_name(index), _get_error_name(index))
                for index, _, _, _ in checked
            ),
            *(pl.col(_get_cell_name(index)) for index, _ in carried),
        )
        .head(max_errors)
        .collect(engine=QUERY_ENGINE)
    )

    _cell_error_adapter = TypeAdapter(CellError)
    row_numbers = check_frame.get_column(NUMBER_COLUMN_NAME).to_list()
    for index, mapping, templates, _ in checked:
        cells = check_frame.get_column(_get_cell_name(index)).to_list()
        error_tags = check_frame.get_column(_get_error_name(index)).to_list()

        for row_number, cell, error_mask in zip(row_numbers, cells, error_tags):
            if not error_mask:
                continue

            for error_tag, template in enumerate(templates):
                if not error_mask >> error_tag & 1:
                    continue
                error_dict = template.model_dump(by_alias=True)
                error_dict["rowNumber"] = row_number
                error_dict["cell"] = str(cell if cell is not None else "")
                errors[mapping.target.name].append(
                    _cell_error_adapter.validate_python(error_dict)
                )

    return errors


def _create_check_pairs(mapping: ColumnMapping, index: int):
    cell_mapping = CellMapping(
        source=pl.col(_get_source_name(index)),
        target=pl.col(_get_target_name(index)),
    )
    return [
        (check_cell, check)
        for check_cell in CHECK_FUNCTIONS
        if (check := check_cell(mapping.target, cell_mapping))
    ]


def _get_is_parsed(mapping: ColumnMapping) -> bool:
    return not normalize_column(mapping).meta.eq(
        normalize_column(mapping, keep_type=True)
    )


def _create_flag_exprs(
    flags: Sequence[FlagFactory], target_names: dict[str, str]
) -> list[pl.Expr]:
    def read_typed(name: str) -> pl.Expr | None:
        target_name = target_names.get(name)
        return pl.col(target_name) if target_name else None

    return [
        create_flag(read_typed).alias(_get_flag_name(index))
        for index, create_flag in enumerate(flags)
    ]


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
