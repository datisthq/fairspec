from __future__ import annotations

import json

from fairspec_dataset import write_temp_file
from fairspec_metadata import Resource

from ..infer import infer_data_schema


class TestInferDataSchema:
    def test_should_infer_schema_from_inline_data(self):
        resource = Resource(data={"name": "test", "age": 25})
        schema = infer_data_schema(resource)
        assert schema is not None
        assert schema.get("type") == "object"

    def test_should_infer_schema_from_inline_array(self):
        resource = Resource(data=[{"id": 1}, {"id": 2}])
        schema = infer_data_schema(resource)
        assert schema is not None

    def test_should_infer_schema_from_json_file(self):
        data = {"name": "test", "value": 42}
        path = write_temp_file(json.dumps(data), format="json")
        resource = Resource(data=path)
        schema = infer_data_schema(resource)
        assert schema is not None
        assert schema.get("type") == "object"

    def test_should_return_none_for_no_data(self):
        resource = Resource()
        schema = infer_data_schema(resource)
        assert schema is None

    def test_should_return_none_for_csv_file(self):
        path = write_temp_file("id,name\n1,english", format="csv")
        resource = Resource(data=path)
        schema = infer_data_schema(resource)
        assert schema is None
