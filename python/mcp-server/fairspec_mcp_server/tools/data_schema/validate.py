from typing import Annotated

from fairspec_library import Descriptor, Report, validate_data_schema
from pydantic import Field

from ...helpers.limit import limit_report
from ...models.tool import ToolResult
from .._shared.schemas import MaxErrors, Truncated


class ValidateDataSchemaResult(ToolResult):
    report: Report
    truncated: Truncated


def validate_data_schema_tool(
    source: Annotated[
        Descriptor | str,
        Field(description="The data schema to validate (descriptor object or file path)"),
    ],
    maxErrors: MaxErrors = None,
) -> ValidateDataSchemaResult:
    """Validate a data schema (JSON Schema) against the Fairspec Data Schema specification. Returns validation report with errors if any."""
    report, truncated = limit_report(validate_data_schema(source), max_errors=maxErrors)
    return ValidateDataSchemaResult(report=report, truncated=truncated)
