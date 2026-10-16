from __future__ import annotations

from fairspec_dataset import write_temp_file
from fairspec_metadata import Resource

from ..infer import infer_file_dialect


class TestInferFileDialect:
    def test_should_infer_csv_dialect(self):
        path = write_temp_file("id,name\n1,english\n2,中文", format="csv")
        resource = Resource(data=path)
        dialect = infer_file_dialect(resource)
        assert dialect is not None

    def test_should_infer_json_dialect(self):
        path = write_temp_file('[{"id": 1}]', format="json")
        resource = Resource(data=path)
        dialect = infer_file_dialect(resource)
        assert dialect is not None
        assert dialect.format == "json"

    def test_should_infer_jsonl_dialect(self):
        path = write_temp_file('{"id": 1}\n{"id": 2}\n', format="jsonl")
        resource = Resource(data=path)
        dialect = infer_file_dialect(resource)
        assert dialect is not None
        assert dialect.format == "jsonl"

    def test_should_infer_xlsx_dialect(self):
        resource = Resource(data="test.xlsx")
        dialect = infer_file_dialect(resource)
        assert dialect is not None

    def test_should_return_none_for_unknown_format(self):
        resource = Resource(data="test.unknown")
        dialect = infer_file_dialect(resource)
        assert dialect is None

    def test_should_return_none_for_no_data(self):
        resource = Resource()
        dialect = infer_file_dialect(resource)
        assert dialect is None
