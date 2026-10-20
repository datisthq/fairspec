from typing import Annotated

from fairspec_library import preview_file
from pydantic import Field

from ...models.tool import ToolResult


class PreviewFileResult(ToolResult):
    text: str = Field(description="The raw file source")
    truncated: bool = Field(description="Whether the file continues past the preview")


def preview_file_tool(
    path: Annotated[str, Field(description="Local or remote path of the file")],
    maxLines: Annotated[
        int | None,
        Field(ge=1, le=200, description="Maximum number of lines (default 20)"),
    ] = None,
    maxBytes: Annotated[
        int | None,
        Field(ge=1, le=20_000, description="Maximum number of bytes (default 10000)"),
    ] = None,
) -> PreviewFileResult:
    """Preview the raw source of a local or remote file as UTF-8 text: the first lines exactly as stored, before any parsing. Use it to see a CSV's delimiter, header and quoting, or a JSON file's structure. Stops at `maxLines` or `maxBytes`, whichever comes first; `truncated` is true when the file continues."""
    preview = preview_file(path, max_lines=maxLines, max_bytes=maxBytes)
    return PreviewFileResult(text=preview.text, truncated=preview.truncated)
