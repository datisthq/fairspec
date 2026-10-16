from __future__ import annotations

import polars as pl
from fairspec_dataset import get_temp_file_path
from fairspec_metadata import ArrowFileDialect, Resource

from ..load import load_arrow_table


class TestLoadArrowTable:
    def test_should_load_local_file(self):
        path = get_temp_file_path()
        pl.DataFrame({"id": [1, 2], "name": ["english", "中文"]}).write_ipc(path)

        table = load_arrow_table(Resource(data=path, fileDialect=ArrowFileDialect()))
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_load_local_file_multipart(self):
        path1 = get_temp_file_path()
        path2 = get_temp_file_path()
        pl.DataFrame({"id": [1, 2], "name": ["english", "中文"]}).write_ipc(path1)
        pl.DataFrame({"id": [1, 2], "name": ["english", "中文"]}).write_ipc(path2)

        table = load_arrow_table(Resource(data=[path1, path2]))
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]
