from __future__ import annotations

from dataclasses import dataclass, field
from typing import TYPE_CHECKING, Literal

import polars as pl
from fairspec_metadata import RowPrimaryKeyError, RowUniqueKeyError

if TYPE_CHECKING:
    from fairspec_table.models import SchemaMapping


@dataclass
class RowKeyCheck:
    key_columns: list[str]
    is_error_expr: pl.Expr
    error_template: str


def create_row_key_checks(mapping: SchemaMapping) -> list[RowKeyCheck]:
    unique_keys = mapping.target.uniqueKeys or []
    primary_key = mapping.target.primaryKey

    checks: list[RowKeyCheck] = []

    if primary_key:
        checks.append(_create_row_key_check(primary_key, key_type="primary"))

    for key in unique_keys:
        checks.append(_create_row_key_check(key, key_type="unique"))

    return checks


@dataclass
class KeyHashProbe:
    """Records the hash of every present key while another query reads the table."""

    flag_expr: pl.Expr
    hashes: list[pl.Series] = field(default_factory=list)

    def get_has_duplicate_key(self) -> bool:
        if not self.hashes:
            return False
        hashes = pl.concat(self.hashes).drop_nulls()
        return hashes.n_unique() < hashes.len()


def create_key_hash_probe(check: RowKeyCheck) -> KeyHashProbe:
    hashes: list[pl.Series] = []
    keys = check.key_columns

    # Called per batch on polars worker threads; the GIL makes the append safe, and polars
    # hashes before the call, so the Python side only keeps the batch. It returns
    # all-false, so the flag joins the reading query's filter without changing its rows
    def record_hashes(batch: pl.Series) -> pl.Series:
        hashes.append(batch)
        return pl.repeat(False, batch.len(), dtype=pl.Boolean, eager=True)

    key_expr = pl.col(keys[0]) if len(keys) == 1 else pl.struct(keys)
    flag_expr = (
        pl.when(create_key_presence_expr(keys))
        .then(key_expr.hash())
        .map_batches(record_hashes, return_dtype=pl.Boolean, is_elementwise=True)
    )
    return KeyHashProbe(flag_expr=flag_expr, hashes=hashes)


def create_key_presence_expr(key_columns: list[str]) -> pl.Expr:
    """A key whose values are all null is never a duplicate."""
    return pl.any_horizontal([pl.col(name).is_not_null() for name in key_columns])


def _create_row_key_check(
    key_columns: list[str],
    *,
    key_type: Literal["primary", "unique"],
) -> RowKeyCheck:
    is_error_expr = pl.struct(
        key_columns
    ).is_first_distinct().not_() & create_key_presence_expr(key_columns)

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
        is_error_expr=is_error_expr,
        error_template=error_model.model_dump_json(),
    )
