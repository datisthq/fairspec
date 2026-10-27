from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import WkbColumn, WkbColumnProperty

from ..wkb import inspect_wkb_column

POINT = "0101000000000000000000f03f0000000000000040"


class TestInspectWkbColumn:
    @pytest.mark.parametrize(
        "cell, valid",
        [
            ("0101000000000000000000f03f0000000000000040", True),
            ("ghijkl", False),
            ("0x1234", False),
            ("hello world", False),
        ],
    )
    def test_wkb_validation(self, cell: str, valid: bool):
        table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
        column = WkbColumn(
            name="name",
            type="wkb",
            property=WkbColumnProperty(),
        )

        errors = inspect_wkb_column(column, table)

        assert (len(errors) == 0) == valid

    @pytest.mark.parametrize(
        "cell, valid",
        [
            ("02" + POINT[2:], False),
            ("ff" + POINT[2:], False),
            ("010400000001000000" + POINT, True),
            ("010400000001000000" + "02" + POINT[2:], False),
        ],
    )
    def test_wkb_unknown_byte_order(self, cell: str, valid: bool):
        table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
        column = WkbColumn(
            name="name",
            type="wkb",
            property=WkbColumnProperty(),
        )

        errors = inspect_wkb_column(column, table)

        assert (len(errors) == 0) == valid
