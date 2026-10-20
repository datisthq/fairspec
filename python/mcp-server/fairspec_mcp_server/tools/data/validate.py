from typing import Annotated

from fairspec_library import Report, Resource, validate_data
from pydantic import Field

from ...models.tool import ToolResult


class ValidateDataResult(ToolResult):
    report: Report


def validate_data_tool(
    resource: Annotated[Resource, Field(description="The data resource to validate")],
) -> ValidateDataResult:
    """Validate data against a data schema (JSON Schema). Returns a validation report with errors if any."""
    return ValidateDataResult(report=validate_data(resource))
