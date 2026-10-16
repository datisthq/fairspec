from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import HexColumn, HexColumnProperty

from ..hex import parse_hex_column


COLUMN = HexColumn(
    name="name",
    type="hex",
    property=HexColumnProperty(),
)


class TestParseHexColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("0123456789abcdef", "0123456789abcdef"),
            ("ABCDEF", "ABCDEF"),
            ("deadbeef", "deadbeef"),
            ("ghijkl", None),
            ("0x1234", None),
            ("hello world", None),
        ],
    )
    def test_parse(self, cell: str, expected: str | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        result = table.select(parse_hex_column(COLUMN, pl.col("name")))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
