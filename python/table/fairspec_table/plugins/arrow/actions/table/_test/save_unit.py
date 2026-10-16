from __future__ import annotations

from datetime import date, datetime, timezone
from zoneinfo import ZoneInfo

import polars as pl
from fairspec_dataset import get_temp_file_path
from fairspec_metadata import Resource

from ..load import load_arrow_table
from ..save import save_arrow_table


class TestSaveArrowTable:
    def test_should_save_table_to_arrow_file(self):
        path = get_temp_file_path()
        source = pl.DataFrame(
            {"id": [1.0, 2.0, 3.0], "name": ["Alice", "Bob", "Charlie"]}
        ).lazy()

        save_arrow_table(source, path=path)

        table = load_arrow_table(Resource(data=path))
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1.0, "name": "Alice"},
            {"id": 2.0, "name": "Bob"},
            {"id": 3.0, "name": "Charlie"},
        ]

    def test_should_save_and_load_various_data_types(self):
        path = get_temp_file_path()
        source = pl.DataFrame(
            [
                pl.Series("boolean", [True], dtype=pl.Boolean),
                pl.Series("date", [date(2025, 1, 1)], dtype=pl.Date),
                pl.Series(
                    "datetime",
                    [datetime(2025, 1, 1, tzinfo=timezone.utc)],
                    dtype=pl.Datetime,
                ),
                pl.Series("integer", [1], dtype=pl.Int32),
                pl.Series("number", [1.1], dtype=pl.Float64),
                pl.Series("string", ["string"], dtype=pl.String),
            ]
        ).lazy()

        save_arrow_table(source, path=path)

        target = load_arrow_table(Resource(data=path), denormalized=True)
        frame: pl.DataFrame = target.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {
                "boolean": True,
                "date": "2025-01-01",
                "datetime": datetime(2025, 1, 1, tzinfo=ZoneInfo("UTC")),
                "integer": 1,
                "number": 1.1,
                "string": "string",
            },
        ]
