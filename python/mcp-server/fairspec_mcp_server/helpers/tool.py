import functools
from collections.abc import Callable

from mcp.server.mcpserver.exceptions import ToolError
from mcp.shared.exceptions import MCPError
from mcp.types import CallToolResult, TextContent

from ..models.tool import ToolResult
from .limit import MAX_RESPONSE_CHARS


def guard_tool[**P](tool: Callable[P, ToolResult]) -> Callable[P, CallToolResult]:
    """Pass a tool's failure to the client with its message, and cap its response size."""

    # MCPServer reads the signature through `__wrapped__`, so the schemas stay the tool's
    @functools.wraps(tool)
    def guarded_tool(*args: P.args, **kwargs: P.kwargs) -> CallToolResult:
        try:
            result = tool(*args, **kwargs)
        except (ToolError, MCPError):
            raise
        except Exception as exc:
            # The SDK hides the message of anything but a ToolError
            raise ToolError(str(exc)) from exc

        text = result.model_dump_json(by_alias=True)
        if len(text) > MAX_RESPONSE_CHARS:
            raise ToolError(
                f"The response exceeds {MAX_RESPONSE_CHARS} characters; narrow the input"
            )

        # The SDK would indent the text copy, doubling its size against the budget
        return CallToolResult(
            content=[TextContent(type="text", text=text)],
            structured_content=result.model_dump(mode="json", by_alias=True),
        )

    return guarded_tool
