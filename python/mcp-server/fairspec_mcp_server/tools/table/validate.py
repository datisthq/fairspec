from typing import Annotated

from fairspec_library import (
    Report,
    Resource,
    ValidateTableOptions,
    validate_table,
)
from pydantic import Field

from ...models.tool import ToolResult


class ValidateTableResult(ToolResult):
    report: Report


def validate_table_tool(
    resource: Annotated[Resource, Field(description="The table resource to validate")],
    options: Annotated[
        ValidateTableOptions | None, Field(description="Validation options")
    ] = None,
) -> ValidateTableResult:
    """Validate a table resource against its schema. Returns a validation report with errors if any."""
    return ValidateTableResult(report=validate_table(resource, **(options or {})))
