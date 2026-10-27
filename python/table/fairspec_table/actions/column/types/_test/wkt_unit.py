from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import WktColumn, WktColumnProperty

from ..wkt import inspect_wkt_column


class TestInspectWktColumn:
    @pytest.mark.parametrize(
        "cell, valid",
        [
            ("POINT (0 0)", True),
            ("MULTIPOINT ((0 0), (1 1))", True),
            ("ghijkl", False),
            ("0x1234", False),
            ("hello world", False),
            ("POINTX (1 2)", False),
            ("MULTIPOLYGONPOINT (((0 0, 1 0, 1 1, 0 0)))", False),
            ("POINT1 (1 2)", False),
            ("pointz (1 2 3)", True),
            ("POINT (1e3 nan)", True),
        ],
    )
    def test_wkt_validation(self, cell: str, valid: bool):
        table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
        column = WktColumn(
            name="name",
            type="wkt",
            property=WktColumnProperty(),
        )

        errors = inspect_wkt_column(column, table)

        assert (len(errors) == 0) == valid
