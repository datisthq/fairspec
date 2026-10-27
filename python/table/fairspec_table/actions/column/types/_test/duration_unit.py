from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import DurationColumn, DurationColumnProperty

from ..duration import inspect_duration_column


COLUMN = DurationColumn(
    name="name",
    type="duration",
    property=DurationColumnProperty(),
)


class TestInspectDurationColumn:
    @pytest.mark.parametrize(
        "cell, valid",
        [
            ("P23DT23H", True),
            ("P1Y2M3DT4H5M6S", True),
            ("PT30M", True),
            ("P1D", True),
            ("PT1H", True),
            ("P1W", True),
            ("PT0S", True),
            ("p1dt2h", True),
            ("-P1D", False),
            ("P1W2D", False),
            ("PT1.5S", False),
            ("P1Y1D", False),
            ("ghijkl", False),
            ("0x1234", False),
            ("hello world", False),
            ("PT", False),
            ("P1DT", False),
            ("P1DT\n", False),
        ],
    )
    def test_inspect(self, cell: str, valid: bool):
        table = pl.DataFrame({"name": [cell]}).lazy()
        errors = inspect_duration_column(COLUMN, table)
        assert (len(errors) == 0) == valid

    def test_inspect_string_constraints_of_readable_cells(self):
        column = DurationColumn(
            name="name",
            type="duration",
            property=DurationColumnProperty(enum=["P1D"], maxLength=3),
        )
        table = pl.DataFrame({"name": ["P1D", "P22D", "1D"]}).lazy()
        errors = inspect_duration_column(column, table)
        assert [(error.type, error.rowNumber) for error in errors] == [
            ("cell/enum", 2),
            ("cell/maxLength", 2),
            ("cell/type", 3),
        ]
