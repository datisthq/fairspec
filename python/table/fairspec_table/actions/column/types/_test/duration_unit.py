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
            ("ghijkl", False),
            ("0x1234", False),
            ("hello world", False),
        ],
    )
    def test_inspect(self, cell: str, valid: bool):
        table = pl.DataFrame({"name": [cell]}).lazy()
        errors = inspect_duration_column(COLUMN, table)
        assert (len(errors) == 0) == valid
