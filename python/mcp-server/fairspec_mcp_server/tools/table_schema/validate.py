from typing import Annotated

from fairspec_library import Descriptor, Report, validate_table_schema
from pydantic import Field

from ...helpers.limit import limit_report
from ...models.tool import ToolResult
from .._shared.schemas import MaxErrors, Truncated


class ValidateTableSchemaResult(ToolResult):
    report: Report
    truncated: Truncated


def validate_table_schema_tool(
    source: Annotated[
        Descriptor | str,
        Field(
            description="The table schema to validate (descriptor object or file path)"
        ),
    ],
    maxErrors: MaxErrors = None,
) -> ValidateTableSchemaResult:
    """Validate a table schema against the Fairspec Table Schema specification. Returns validation report with errors if any."""
    report, truncated = limit_report(validate_table_schema(source), max_errors=maxErrors)
    return ValidateTableSchemaResult(report=report, truncated=truncated)
