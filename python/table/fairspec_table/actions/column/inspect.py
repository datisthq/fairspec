from __future__ import annotations


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


def inspect_column(
    mapping: ColumnMapping,
    table: Table,
    *,
    max_errors: int,
) -> list[TableError]:
    errors: list[TableError] = []

    type_errors = _inspect_type(mapping)
    errors.extend(type_errors)

    if not type_errors:
        data_errors = _inspect_cells(mapping, table, max_errors=max_errors)
        errors.extend(data_errors)

    return errors


COMPAT_MAPPING: dict[type[pl.DataType], list[str]] = {
    pl.Boolean: ["boolean"],
    pl.Categorical: ["string"],
    pl.Date: ["date"],
    pl.Datetime: ["date-time"],
    pl.Float32: ["number", "integer"],
    pl.Float64: ["number", "integer"],
    pl.Int8: ["integer", "number"],
    pl.Int16: ["integer", "number"],
    pl.Int32: ["integer", "number"],
    pl.Int64: ["integer", "number"],
    pl.List: ["list"],
    pl.String: ["unknown"],
    pl.Time: ["time"],
    pl.UInt8: ["integer", "number"],
    pl.UInt16: ["integer", "number"],
    pl.UInt32: ["integer", "number"],
    pl.UInt64: ["integer", "number"],
}


def _inspect_type(mapping: ColumnMapping) -> list[TableError]:
    errors: list[TableError] = []

    compat_types: list[str] = []
    for dtype_cls, types in COMPAT_MAPPING.items():
        if issubclass(mapping.source.type, dtype_cls):
            compat_types = types
            break

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


def _inspect_cells(
    mapping: ColumnMapping,
    table: Table,
    *,
    max_errors: int,
) -> list[TableError]:
    target = mapping.target
    match target.type:
        case "duration":
            return inspect_duration_column(target, table)  # type: ignore[arg-type, return-type]
        case "wkb":
            return inspect_wkb_column(target, table)  # type: ignore[arg-type, return-type]
        case "wkt":
            return inspect_wkt_column(target, table)  # type: ignore[arg-type, return-type]
        case "array":
            return inspect_array_column(target, table)  # type: ignore[arg-type, return-type]
        case "object":
            return inspect_object_column(target, table)  # type: ignore[arg-type, return-type]
        case "geojson":
            return inspect_geojson_column(target, table)  # type: ignore[arg-type, return-type]
        case "topojson":
            return inspect_topojson_column(target, table)  # type: ignore[arg-type, return-type]
        case _:
            return _inspect_cells_in_polars(mapping, table, max_errors=max_errors)


def _inspect_cells_in_polars(
    mapping: ColumnMapping,
    table: Table,
    *,
    max_errors: int,
) -> list[TableError]:
    errors: list[TableError] = []

    # The error column carries a tag, not the error itself: a literal is broadcast to
    # every row, so a JSON template would materialize a full-length string column per
    # check. Templates are resolved back in Python once the frame is filtered down.
    error_templates: list[CellError] = []

    column_check_table = table.with_row_index(NUMBER_COLUMN_NAME, 1).select(
        pl.col(NUMBER_COLUMN_NAME),
        normalize_column(mapping).alias("target"),
        normalize_column(mapping, keep_type=True).alias("source"),
        pl.lit(None, dtype=pl.UInt8).alias("error"),
    )

    check_functions = [
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

    for check_cell in check_functions:
        cell_mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        check = check_cell(mapping.target, cell_mapping)
        if not check:
            continue

        error_tag = len(error_templates)
        error_templates.append(check.error_template)

        column_check_table = column_check_table.with_columns(
            pl.when(pl.col("error").is_not_null())
            .then(pl.col("error"))
            .when(check.is_error_expr)
            .then(pl.lit(error_tag, dtype=pl.UInt8))
            .otherwise(pl.lit(None, dtype=pl.UInt8))
            .alias("error"),
        )

    column_check_frame: pl.DataFrame = (  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        column_check_table.filter(pl.col("error").is_not_null())
        .drop("target")
        .head(max_errors)
        .collect(engine=QUERY_ENGINE)
    )

    _cell_error_adapter = TypeAdapter(CellError)
    for row in column_check_frame.to_dicts():
        error_dict = error_templates[row["error"]].model_dump(by_alias=True)
        error_dict["rowNumber"] = row[NUMBER_COLUMN_NAME]
        error_dict["cell"] = str(row["source"] if row["source"] is not None else "")
        errors.append(_cell_error_adapter.validate_python(error_dict))

    return errors
