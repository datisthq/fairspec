from __future__ import annotations

import pytest

from ..format import DATA_SCHEMA_FORMATS


class TestDataSchemaFormats:
    @pytest.mark.parametrize(
        "name, value, expected",
        [
            ("date", "2024-02-29", True),
            ("date", "2023-02-29", False),
            ("date", "2024-13-01", False),
            ("time", "23:59:60Z", True),
            ("time", "15:59:60-08:00", True),
            ("time", "12:00:60Z", False),
            ("time", "12:00:00", False),
            ("date-time", "2024-01-31t10:00:00.5+01:00", True),
            ("date-time", "2024-01-31 10:00:00Z", False),
            ("duration", "P1W", True),
            ("duration", "P", False),
            ("email", "joe@example.com", True),
            ("email", "joe@example.com\n", False),
            ("email", "joe", False),
            ("ipv4", "192.168.0.1", True),
            ("ipv4", "01.1.1.1", False),
            ("uuid", "123E4567-e89b-12d3-a456-426614174000", True),
            ("uuid", "123e4567e89b12d3a456426614174000", False),
        ],
    )
    def test_checks_a_value(self, name: str, value: str, expected: bool):
        assert DATA_SCHEMA_FORMATS[name](value) is expected
