from __future__ import annotations

from fairspec_dataset import write_temp_file
from fairspec_metadata import Resource

from ..infer import infer_resource


class TestInferResource:
    def test_should_infer_name(self):
        path = write_temp_file("id,name\n1,english", format="csv")
        resource = Resource(data=path)
        result = infer_resource(resource)
        assert result.name is not None

    def test_should_infer_file_dialect(self):
        path = write_temp_file("id,name\n1,english", format="csv")
        resource = Resource(data=path)
        result = infer_resource(resource)
        assert result.fileDialect is not None

    def test_should_not_overwrite_existing_name(self):
        path = write_temp_file("id,name\n1,english", format="csv")
        resource = Resource(data=path, name="custom")
        result = infer_resource(resource)
        assert result.name == "custom"

    def test_should_not_mutate_original(self):
        path = write_temp_file("id,name\n1,english", format="csv")
        resource = Resource(data=path)
        result = infer_resource(resource)
        assert result is not resource

    def test_should_handle_resource_number(self):
        resource = Resource(data=[{"id": 1}])
        result = infer_resource(resource, resource_number=5)
        assert result.name == "resource5"
