from __future__ import annotations

import json

from fairspec_dataset import write_temp_file
from fairspec_metadata import Resource

from ..load import load_data


class TestLoadData:
    def test_should_return_inline_data(self):
        resource = Resource(data=[{"id": 1}, {"id": 2}])
        result = load_data(resource)
        assert result == [{"id": 1}, {"id": 2}]

    def test_should_return_inline_object(self):
        resource = Resource(data={"key": "value"})
        result = load_data(resource)
        assert result == {"key": "value"}

    def test_should_load_json_file(self):
        data = {"key": "value"}
        path = write_temp_file(json.dumps(data), format="json")
        resource = Resource(data=path)
        result = load_data(resource)
        assert result == data

    def test_should_load_json_file_with_top_level_array(self):
        data = [{"id": 1, "name": "A"}, {"id": 2, "name": "B"}]
        path = write_temp_file(json.dumps(data), format="json")
        resource = Resource(data=path)
        result = load_data(resource)
        assert result == data

    def test_should_return_none_for_non_json_file(self):
        path = write_temp_file("id,name\n1,english", format="csv")
        resource = Resource(data=path)
        result = load_data(resource)
        assert result is None

    def test_should_return_none_for_empty_resource(self):
        resource = Resource()
        result = load_data(resource)
        assert result is None
