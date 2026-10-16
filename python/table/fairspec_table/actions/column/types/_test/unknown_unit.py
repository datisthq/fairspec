from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import UnknownColumn, UnknownColumnProperty

from ..unknown import stringify_unknown_column


COLUMN = UnknownColumn(
    name="name",
    type="unknown",
    property=UnknownColumnProperty(),
)


class TestStringifyUnknownColumn:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (1.0, "1.0"),
            (3.14, "3.14"),
            (True, "true"),
            (False, "false"),
            ("text", "text"),
        ],
    )
    def test_stringify(self, value: object, expected: str):
        table = pl.DataFrame({"name": [value]}).lazy()
        result = table.select(stringify_unknown_column(COLUMN, pl.col("name")))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
