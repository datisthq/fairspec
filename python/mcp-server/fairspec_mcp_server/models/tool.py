from fairspec_library import FairspecModel
from pydantic import SerializationInfo, SerializerFunctionWrapHandler, model_serializer


class ToolResult(FairspecModel):
    """The structured output of a tool, serialized without unset optional fields."""

    # mcp dumps tool output without exclude_none, so every unset field would reach the client as null
    @model_serializer(mode="wrap")
    def serialize_tool_result(
        self, handler: SerializerFunctionWrapHandler, info: SerializationInfo
    ):
        if info.exclude_none:
            return handler(self)
        return self.model_dump(mode=info.mode, by_alias=info.by_alias, exclude_none=True)
