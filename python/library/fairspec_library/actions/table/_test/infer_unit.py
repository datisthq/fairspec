from __future__ import annotations

import polars as pl
from fairspec_dataset import write_temp_file
from fairspec_metadata import Resource

from ..infer import infer_table


class TestInferTable:
    def test_should_infer_and_load_table(self):
        path = write_temp_file("id,name\n1,english\n2,中文", format="csv")
        resource = Resource(data=path)
        table = infer_table(resource)
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(frame) == 2

    def test_should_return_none_for_empty_resource(self):
        resource = Resource()
        table = infer_table(resource)
        assert table is None
