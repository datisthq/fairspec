from __future__ import annotations

from fairspec_dataset import write_temp_file
from fairspec_metadata import Resource

from ..infer import infer_table_schema


class TestInferTableSchema:
    def test_should_infer_schema_from_csv(self):
        path = write_temp_file("id,name\n1,english\n2,中文", format="csv")
        resource = Resource(data=path)
        schema = infer_table_schema(resource)
        assert schema is not None
        assert schema.properties is not None

    def test_should_return_none_for_empty_resource(self):
        resource = Resource()
        schema = infer_table_schema(resource)
        assert schema is None

    def test_should_infer_schema_from_inline_data(self):
        resource = Resource(
            data=[{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}]
        )
        schema = infer_table_schema(resource)
        assert schema is not None
