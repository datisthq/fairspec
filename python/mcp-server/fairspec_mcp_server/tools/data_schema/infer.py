from typing import Annotated

from fairspec_library import DataSchema, Resource, infer_data_schema
from pydantic import Field

from ...models.tool import ToolResult


class InferDataSchemaResult(ToolResult):
    dataSchema: DataSchema | None = None


def infer_data_schema_tool(
    resource: Annotated[
        Resource, Field(description="The data resource to infer schema from")
    ],
) -> InferDataSchemaResult:
    """Infer a data schema (JSON Schema) from data. Analyzes the data structure to determine types and properties."""
    return InferDataSchemaResult(dataSchema=infer_data_schema(resource))
