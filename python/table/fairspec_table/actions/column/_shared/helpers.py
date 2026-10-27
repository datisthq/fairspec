from __future__ import annotations

import json
from typing import Callable, Union, cast

import polars as pl

from fairspec_metadata import (
    ArrayColumn,
    CellJsonError,
    CellMissingError,
    CellTypeError,
    DurationColumn,
    GeojsonColumn,
    ObjectColumn,
    TopojsonColumn,
    WkbColumn,
    WktColumn,
    inspect_json,
)
from fairspec_metadata import ColumnType
from fairspec_metadata import CellError

from fairspec_table.models import ColumnMapping, PolarsColumn, Table
from fairspec_table.settings import QUERY_ENGINE, NUMBER_COLUMN_NAME

from ..substitute import substitute_column


def inspect_text_column(
    column: Union[DurationColumn, WktColumn, WkbColumn],
    table: Table,
    *,
    parse: Callable[[str], object],
    mapping: ColumnMapping | None = None,
) -> list[CellError]:
    errors: list[CellError] = []
    frame = _collect_cell_frame(table, mapping or _create_text_mapping(column))

    for row in frame.to_dicts():
        if row["source"] is None:
            if not column.nullable:
                errors.append(_create_missing_error(column.name, row))
            continue

        target = None
        try:
            target = parse(row["source"])
        except Exception:
            pass

        if target is None:
            errors.append(
                CellTypeError(
                    type="cell/type",
                    cell=str(row["source"]),
                    columnName=column.name,
                    columnType=ColumnType(column.type),
                    rowNumber=row[NUMBER_COLUMN_NAME],
                )
            )

    return errors


def inspect_json_column(
    column: Union[ArrayColumn, ObjectColumn, GeojsonColumn, TopojsonColumn],
    table: Table,
    *,
    type_json_schema: dict[str, object] | None = None,
    mapping: ColumnMapping | None = None,
) -> list[CellError]:
    errors: list[CellError] = []

    column_type = ColumnType(column.type)
    constraint_json_schema = column.property.model_dump(exclude_none=True, by_alias=True)
    needs_validator = bool(type_json_schema) or bool(
        constraint_json_schema.keys() - ANNOTATION_KEYWORDS
    )

    mapping = mapping or _create_text_mapping(column)
    cell_frame = _select_cell_frame(table, mapping).with_columns(
        _create_json_kind_expr(mapping, pl.col("source")).alias("valid")
    )
    if not needs_validator:
        cell_frame = cell_frame.filter(
            pl.col("source").is_null() & (not column.nullable)
            | pl.col("source").is_not_null() & ~pl.col("valid")
        )
    frame = cast(pl.DataFrame, cell_frame.collect(engine=QUERY_ENGINE))

    for row in frame.to_dicts():
        if row["source"] is None:
            if not column.nullable:
                errors.append(_create_missing_error(column.name, row))
            continue

        if not row["valid"]:
            errors.append(
                CellTypeError(
                    type="cell/type",
                    cell=str(row["source"]),
                    columnName=column.name,
                    columnType=column_type,
                    rowNumber=row[NUMBER_COLUMN_NAME],
                )
            )
            continue

        target = json.loads(row["source"])
        if type_json_schema:
            format_errors = inspect_json(target, json_schema=type_json_schema)

            if format_errors:
                errors.append(
                    CellTypeError(
                        type="cell/type",
                        cell=str(row["source"]),
                        columnName=column.name,
                        columnType=column_type,
                        rowNumber=row[NUMBER_COLUMN_NAME],
                    )
                )
            continue

        if constraint_json_schema:
            constraint_errors = inspect_json(target, json_schema=constraint_json_schema)

            for error in constraint_errors:
                errors.append(
                    CellJsonError(
                        type="cell/json",
                        cell=str(row["source"]),
                        columnName=column.name,
                        rowNumber=row[NUMBER_COLUMN_NAME],
                        message=error["message"],
                        jsonPointer=error["jsonPointer"],
                    )
                )

    return errors


# Keywords that never fail a value: a column with only these needs no validator
ANNOTATION_KEYWORDS = frozenset(
    {
        "type",
        "title",
        "description",
        "examples",
        "default",
        "$comment",
        "deprecated",
        "readOnly",
        "writeOnly",
        "rdfType",
        "missingValues",
    }
)


def _create_text_mapping(
    column: Union[
        ArrayColumn,
        DurationColumn,
        GeojsonColumn,
        ObjectColumn,
        TopojsonColumn,
        WkbColumn,
        WktColumn,
    ],
) -> ColumnMapping:
    return ColumnMapping(
        source=PolarsColumn(name=column.name, type=pl.String), target=column
    )


def _collect_cell_frame(table: Table, mapping: ColumnMapping) -> pl.DataFrame:
    return cast(
        pl.DataFrame, _select_cell_frame(table, mapping).collect(engine=QUERY_ENGINE)
    )


def _select_cell_frame(table: Table, mapping: ColumnMapping) -> pl.LazyFrame:
    return table.with_row_index(NUMBER_COLUMN_NAME, 1).select(
        pl.col(NUMBER_COLUMN_NAME),
        _encode_nested_column(
            mapping, substitute_column(mapping, pl.col(mapping.source.name))
        ).alias("source"),
        _encode_nested_column(mapping, pl.col(mapping.source.name)).alias("cell"),
    )


# Polars' parser is the one judge of a JSON cell, so every library rejects the same text
def _create_json_kind_expr(mapping: ColumnMapping, source_expr: pl.Expr) -> pl.Expr:
    if issubclass(mapping.source.type, (pl.Struct, pl.List)):
        return pl.lit(True)
    opener = r"^[ \t\n\r]*\[" if mapping.target.type == "array" else r"^[ \t\n\r]*\{"
    return (
        source_expr.str.contains(opener)
        & source_expr.str.json_path_match("$").is_not_null()
    )


# Wrapping in a one-field struct lets List encode too, which has no json_encode
def _encode_nested_column(mapping: ColumnMapping, column_expr: pl.Expr) -> pl.Expr:
    if not issubclass(mapping.source.type, (pl.Struct, pl.List)):
        return column_expr
    encoded = pl.struct(column_expr.alias("v")).struct.json_encode()
    return pl.when(column_expr.is_not_null()).then(
        encoded.str.replace(r'(?s)^\{"v":(.*)\}$', "${1}")
    )


def _create_missing_error(column_name: str, row: dict[str, object]) -> CellMissingError:
    return CellMissingError(
        type="cell/missing",
        columnName=column_name,
        rowNumber=cast(int, row[NUMBER_COLUMN_NAME]),
        cell=str(row["cell"] if row["cell"] is not None else ""),
    )
