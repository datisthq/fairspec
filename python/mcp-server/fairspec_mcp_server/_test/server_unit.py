import pytest
from fairspec_library import write_temp_file
from mcp import Client

from ..server import server

pytestmark = pytest.mark.anyio


@pytest.fixture
def anyio_backend():
    return "asyncio"


class TestServer:
    async def test_should_list_tools_with_typescript_names(self):
        async with Client(server) as client:
            result = await client.list_tools()

        assert sorted(tool.name for tool in result.tools) == sorted(
            [
                "inferDataSchemaTool",
                "inferDatasetTool",
                "inferFileDialectTool",
                "inferTableSchemaTool",
                "queryTableTool",
                "validateDataSchemaTool",
                "validateDatasetTool",
                "validateDataTool",
                "validateTableSchemaTool",
                "validateTableTool",
            ]
        )
        for tool in result.tools:
            assert tool.description
            assert tool.output_schema

    async def test_should_call_tool_with_structured_output(self, capfd):
        path = write_temp_file("id,name\n1,alice\n2,bob\n", format="csv")

        async with Client(server) as client:
            result = await client.call_tool(
                "inferTableSchemaTool", {"resource": {"data": path}}
            )

        assert not result.is_error
        assert capfd.readouterr().out == ""
        assert result.structured_content == {
            "tableSchema": {
                "properties": {"id": {"type": "integer"}, "name": {"type": "string"}}
            }
        }

    async def test_should_report_tool_failure_as_error(self):
        async with Client(server) as client:
            result = await client.call_tool(
                "queryTableTool", {"resource": {}, "query": "SELECT * FROM self"}
            )

        assert result.is_error
