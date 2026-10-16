from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import EmailColumn, EmailColumnProperty

from ..email import parse_email_column


COLUMN = EmailColumn(
    name="name",
    type="email",
    property=EmailColumnProperty(),
)


class TestParseEmailColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("user@example.com", "user@example.com"),
            ("test.name@domain.org", "test.name@domain.org"),
            ("user+tag@example.co.uk", "user+tag@example.co.uk"),
            ("", None),
            ("invalid", None),
            ("@example.com", None),
            ("user@", None),
            ("user example.com", None),
        ],
    )
    def test_parse(self, cell: str, expected: str | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        result = table.select(parse_email_column(COLUMN, pl.col("name")))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
