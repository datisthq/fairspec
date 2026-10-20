import sys

import pytest
from fairspec_library import write_temp_file
from mcp import Client, StdioServerParameters

pytestmark = pytest.mark.anyio

SERVER = StdioServerParameters(
    command=sys.executable,
    args=["-c", "from fairspec_terminal.main import main; main()", "mcp"],
)


@pytest.fixture
def anyio_backend():
    return "asyncio"


class TestMcp:
    async def test_should_serve_tools_over_stdio(self):
        async with Client(SERVER) as client:
            result = await client.list_tools()

        assert "inferTableSchemaTool" in [tool.name for tool in result.tools]

    async def test_should_call_tool_over_stdio(self):
        path = write_temp_file("id,name\n1,alice\n2,bob\n", format="csv")

        async with Client(SERVER) as client:
            result = await client.call_tool(
                "inferTableSchemaTool", {"resource": {"data": path}}
            )

        assert not result.is_error
        assert result.structured_content == {
            "tableSchema": {
                "properties": {"id": {"type": "integer"}, "name": {"type": "string"}}
            }
        }

    async def test_should_report_real_message_of_failed_tool_over_stdio(self):
        async with Client(SERVER) as client:
            result = await client.call_tool(
                "validateDatasetTool", {"source": "missing/dataset.json"}
            )

        assert result.is_error
        assert "missing" in str(result.content)
