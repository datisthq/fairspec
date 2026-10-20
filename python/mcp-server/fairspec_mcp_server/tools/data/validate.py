from typing import Annotated

from fairspec_library import Report, Resource, validate_data
from pydantic import Field

from ...helpers.limit import limit_report
from ...models.tool import ToolResult
from .._shared.schemas import MaxErrors, Truncated


class ValidateDataResult(ToolResult):
    report: Report
    truncated: Truncated


def validate_data_tool(
    resource: Annotated[Resource, Field(description="The data resource to validate")],
    maxErrors: MaxErrors = None,
) -> ValidateDataResult:
    """Validate data against a data schema (JSON Schema). Returns a validation report with errors if any."""
    report, truncated = limit_report(validate_data(resource), max_errors=maxErrors)
    return ValidateDataResult(report=report, truncated=truncated)
