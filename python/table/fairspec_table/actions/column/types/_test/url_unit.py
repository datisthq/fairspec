from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import UrlColumn, UrlColumnProperty

from ..url import parse_url_column


COLUMN = UrlColumn(
    name="name",
    type="url",
    property=UrlColumnProperty(),
)


class TestParseUrlColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("https://example.com", "https://example.com"),
            ("http://example.com", "http://example.com"),
            ("https://example.com/path?query=1", "https://example.com/path?query=1"),
            ("", None),
            ("example.com", None),
            ("ftp://example.com", None),
            ("not a url", None),
        ],
    )
    def test_parse(self, cell: str, expected: str | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        result = table.select(parse_url_column(COLUMN, pl.col("name")))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
