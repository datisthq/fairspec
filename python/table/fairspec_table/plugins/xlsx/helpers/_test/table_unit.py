from __future__ import annotations

import datetime

import pytest

from ..table import format_sheet_cell


class TestFormatSheetCell:
    @pytest.mark.parametrize(
        ("value", "text"),
        [
            (1.0, "1"),
            (1e21, "1e+21"),
            (1e20, "100000000000000000000"),
            (1e-7, "1e-7"),
            (0.000001, "0.000001"),
            (-0.0, "0"),
            (123.456, "123.456"),
            (-1.5e-10, "-1.5e-10"),
            (0.1 + 0.2, "0.30000000000000004"),
            (5e-324, "5e-324"),
            (1.7976931348623157e308, "1.7976931348623157e+308"),
        ],
    )
    def test_should_format_numbers_as_javascript_does(self, value: float, text: str):
        assert format_sheet_cell(value) == text

    def test_should_format_dates_as_iso(self):
        assert format_sheet_cell(datetime.datetime(2024, 1, 2)) == "2024-01-02T00:00:00"
        assert (
            format_sheet_cell(datetime.datetime(2024, 1, 2, 3, 4, 5, 678000))
            == "2024-01-02T03:04:05.678"
        )

    def test_should_format_booleans_in_lower_case(self):
        assert format_sheet_cell(True) == "true"
        assert format_sheet_cell(False) == "false"
