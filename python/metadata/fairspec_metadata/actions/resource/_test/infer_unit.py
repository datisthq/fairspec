from fairspec_metadata import Resource

from ..infer import infer_resource_name


class TestInferResourceName:
    def test_infers_name_from_single_path(self):
        assert infer_resource_name(Resource(data="/data/users.csv")) == "users"

    def test_infers_name_from_first_path_in_array(self):
        resource = Resource(data=["/data/users.csv", "/data/backup.csv"])
        assert infer_resource_name(resource) == "users"

    def test_infers_name_from_url(self):
        resource = Resource(data="https://example.com/data/products.json")
        assert infer_resource_name(resource) == "products"

    def test_returns_default_name_when_no_path(self):
        assert infer_resource_name(Resource()) == "resource"

    def test_returns_default_name_when_no_filename(self):
        assert infer_resource_name(Resource(data="/data/folder/")) == "resource"

    def test_handles_complex_filename(self):
        resource = Resource(data="/data/file.backup.csv")
        assert infer_resource_name(resource) == "file_backup"

    def test_slugifies_filename(self):
        resource = Resource(data="/data/My Data File!.csv")
        assert infer_resource_name(resource) == "my_data_file"

    def test_returns_numbered_default(self):
        assert infer_resource_name(Resource(), resource_number=1) == "resource1"
