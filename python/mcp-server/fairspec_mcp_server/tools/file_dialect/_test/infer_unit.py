from fairspec_library import Resource, write_temp_file

from ..infer import infer_file_dialect_tool


class TestInferFileDialectTool:
    def test_should_infer_csv_dialect(self):
        path = write_temp_file("id;name\n1;alice\n2;bob\n", format="csv")

        result = infer_file_dialect_tool(Resource(data=path))

        assert result.fileDialect is not None
        dialect = result.fileDialect.model_dump(exclude_none=True)
        assert dialect["format"] == "csv"
        assert dialect["delimiter"] == ";"

    def test_should_return_none_for_inline_data(self):
        result = infer_file_dialect_tool(Resource(data=[{"id": 1}]))

        assert result.fileDialect is None

    def test_should_serialize_without_unset_fields(self):
        path = write_temp_file("id,name\n1,alice\n", format="csv")

        result = infer_file_dialect_tool(Resource(data=path))
        dump = result.model_dump(mode="json", by_alias=True)

        assert "$schema" not in dump["fileDialect"]
        assert None not in dump["fileDialect"].values()
