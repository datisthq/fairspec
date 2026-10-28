from __future__ import annotations

from dataclasses import dataclass, field
from typing import TYPE_CHECKING, Callable, Literal

import polars as pl
from fairspec_metadata import RowPrimaryKeyError, RowUniqueKeyError, get_columns

from fairspec_table.actions.column.normalize import normalize_column
from fairspec_table.models import ColumnMapping

from .._shared.helpers import merge_missing_values

if TYPE_CHECKING:
    from fairspec_table.models import SchemaMapping


@dataclass
class RowKeyCheck:
    key_columns: list[str]
    key_exprs: list[pl.Expr]
    is_error_expr: pl.Expr
    error_template: str


def create_row_key_checks(mapping: SchemaMapping) -> list[RowKeyCheck]:
    names = {column.name for column in mapping.source.columns}
    unique_keys = [
        key for key in mapping.target.uniqueKeys or [] if all(n in names for n in key)
    ]
    primary_key = mapping.target.primaryKey

    checks: list[RowKeyCheck] = []

    if primary_key and all(name in names for name in primary_key):
        checks.append(_create_row_key_check(mapping, primary_key, key_type="primary"))

    for key in unique_keys:
        checks.append(_create_row_key_check(mapping, key, key_type="unique"))

    return checks


# Reads a column's typed value from a query that already parses it, or None
TypedColumnReader = Callable[[str], "pl.Expr | None"]


@dataclass
class KeyHashProbe:
    """Records the hash of every present key while another query reads the table."""

    create_flag_expr: Callable[[TypedColumnReader], pl.Expr]
    hashes: list[pl.Series] = field(default_factory=list)

    def get_has_duplicate_key(self) -> bool:
        if not self.hashes:
            return False
        hashes = pl.concat(self.hashes).drop_nulls()
        return hashes.n_unique() < hashes.len()


def create_key_hash_probe(check: RowKeyCheck) -> KeyHashProbe:
    hashes: list[pl.Series] = []
    falses: dict[int, pl.Series] = {}

    # Called per batch on polars worker threads; the GIL makes the append safe, and polars
    # hashes before the call, so the Python side only keeps the batch. It returns
    # all-false, so the flag joins the reading query's filter without changing its rows;
    # the result is cached by length, as a polars call here waits ~1ms to regain the GIL
    def record_hashes(batch: pl.Series) -> pl.Series:
        hashes.append(batch)
        size = len(batch)
        result = falses.get(size)
        if result is None:
            result = falses[size] = pl.repeat(False, size, dtype=pl.Boolean, eager=True)
        return result

    # A key column the reading query already parses is read from it: the streaming
    # engine does not share a repeated expression, so parsing it again costs a second read
    def create_flag_expr(read_typed: TypedColumnReader) -> pl.Expr:
        keys = [
            typed if (typed := read_typed(name)) is not None else key
            for name, key in zip(check.key_columns, check.key_exprs, strict=True)
        ]
        key_expr = keys[0] if len(keys) == 1 else pl.struct(keys)
        # py-polars passes a constant batch, such as a parsed all-`true` key, as one row;
        # adding the raw cell's null flag, 0 wherever the key is present, keeps every row
        row_expr = pl.col(check.key_columns[0]).is_null().cast(pl.UInt64)
        return (
            pl.when(create_key_presence_expr(keys))
            .then(key_expr.hash() + row_expr)
            .map_batches(record_hashes, return_dtype=pl.Boolean, is_elementwise=True)
        )

    return KeyHashProbe(create_flag_expr=create_flag_expr, hashes=hashes)


def create_key_presence_expr(key_exprs: list[pl.Expr]) -> pl.Expr:
    """A key with any null value is never a duplicate."""
    return pl.all_horizontal([expr.is_not_null() for expr in key_exprs])


def _create_row_key_check(
    mapping: SchemaMapping,
    key_columns: list[str],
    *,
    key_type: Literal["primary", "unique"],
) -> RowKeyCheck:
    key_exprs = [_create_key_column_expr(mapping, name) for name in key_columns]
    is_error_expr = pl.struct(
        key_exprs
    ).is_first_distinct().not_() & create_key_presence_expr(key_exprs)

    error_model: RowPrimaryKeyError | RowUniqueKeyError
    if key_type == "primary":
        error_model = RowPrimaryKeyError(
            type="row/primaryKey", columnNames=key_columns, rowNumber=0
        )
    else:
        error_model = RowUniqueKeyError(
            type="row/uniqueKey", columnNames=key_columns, rowNumber=0
        )

    return RowKeyCheck(
        key_columns=key_columns,
        key_exprs=key_exprs,
        is_error_expr=is_error_expr,
        error_template=error_model.model_dump_json(),
    )


# Keys compare the typed values, so `1.0` and `1` in a number column are one key
def _create_key_column_expr(mapping: SchemaMapping, name: str) -> pl.Expr:
    source = next((c for c in mapping.source.columns if c.name == name), None)
    columns = get_columns(mapping.target.model_dump())
    column = next((c for c in columns if c.name == name), None)
    if not source or not column:
        return pl.col(name)

    target = merge_missing_values(column, mapping.target)
    return normalize_column(ColumnMapping(source=source, target=target))
