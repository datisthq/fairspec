from typing import Annotated

from fairspec_library import (
    InferTableSchemaOptions,
    Resource,
    TableSchema,
    infer_table_schema,
)
from pydantic import Field

from ...models.tool import ToolResult


class InferTableSchemaResult(ToolResult):
    tableSchema: TableSchema | None = None


def infer_table_schema_tool(
    resource: Annotated[
        Resource, Field(description="The table resource to infer schema from")
    ],
    options: Annotated[
        InferTableSchemaOptions | None, Field(description="Schema inference options")
    ] = None,
) -> InferTableSchemaResult:
    """Infer a table schema from table resource data. Analyzes the data to determine column types and constraints."""
    table_schema = infer_table_schema(resource, **(options or {}))
    return InferTableSchemaResult(tableSchema=table_schema)
