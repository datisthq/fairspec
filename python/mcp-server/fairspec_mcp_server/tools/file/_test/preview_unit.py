import pytest
from fairspec_library import write_temp_file
from mcp import Client

from ....server import server
from ..preview import preview_file_tool


@pytest.fixture
def anyio_backend():
    return "asyncio"


class TestPreviewFileTool:
    def test_should_preview_csv_file(self):
        path = write_temp_file("id,name\n1,alice\n2,bob\n", format="csv")

        result = preview_file_tool(path, maxLines=2)

        assert result.text == "id,name\n1,alice\n"
        assert result.truncated is True

    @pytest.mark.anyio
    @pytest.mark.parametrize(
        "limits", [{"maxLines": 201}, {"maxBytes": 20_001}, {"maxBytes": 0}]
    )
    async def test_should_reject_limits_over_cap(self, limits):
        path = write_temp_file("id\n1\n", format="csv")

        async with Client(server) as client:
            result = await client.call_tool("previewFileTool", {"path": path, **limits})

        assert result.is_error
