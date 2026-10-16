from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import Base64Column, Base64ColumnProperty

from ..base64 import parse_base64_column


COLUMN = Base64Column(
    name="name",
    type="base64",
    property=Base64ColumnProperty(),
)


class TestParseBase64Column:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("SGVsbG8gV29ybGQ=", "SGVsbG8gV29ybGQ="),
            ("YWJjZGVm", "YWJjZGVm"),
            ("!!!invalid!!!", None),
            ("not base64", None),
        ],
    )
    def test_parse(self, cell: str, expected: str | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        result = table.select(parse_base64_column(COLUMN, pl.col("name")))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
