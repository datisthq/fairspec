from typing import Annotated

from fairspec_library import Descriptor, Report, validate_table_schema
from pydantic import Field

from ...models.tool import ToolResult


class ValidateTableSchemaResult(ToolResult):
    report: Report


def validate_table_schema_tool(
    source: Annotated[
        Descriptor | str,
        Field(
            description="The table schema to validate (descriptor object or file path)"
        ),
    ],
) -> ValidateTableSchemaResult:
    """Validate a table schema against the Fairspec Table Schema specification. Returns validation report with errors if any."""
    result = validate_table_schema(source)
    return ValidateTableSchemaResult(
        report=Report(valid=result.valid, errors=result.errors)
    )
