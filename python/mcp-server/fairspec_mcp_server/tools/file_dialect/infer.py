from typing import Annotated

from fairspec_library import (
    FileDialect,
    InferFileDialectOptions,
    Resource,
    infer_file_dialect,
)
from pydantic import Field

from ...models.tool import ToolResult


class InferFileDialectResult(ToolResult):
    fileDialect: FileDialect | None = None


def infer_file_dialect_tool(
    resource: Annotated[
        Resource, Field(description="The resource to infer dialect from")
    ],
    options: Annotated[
        InferFileDialectOptions | None, Field(description="Dialect inference options")
    ] = None,
) -> InferFileDialectResult:
    """Infer the dialect (format and format-specific properties) from a resource. Analyzes the data to determine CSV delimiters, JSON structure, etc."""
    file_dialect = infer_file_dialect(resource, **(options or {}))
    return InferFileDialectResult(fileDialect=file_dialect)
