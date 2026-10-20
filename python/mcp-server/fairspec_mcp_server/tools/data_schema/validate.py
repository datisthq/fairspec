from typing import Annotated

from fairspec_library import Descriptor, Report, validate_data_schema
from pydantic import Field

from ...models.tool import ToolResult


class ValidateDataSchemaResult(ToolResult):
    report: Report


def validate_data_schema_tool(
    source: Annotated[
        Descriptor | str,
        Field(description="The data schema to validate (descriptor object or file path)"),
    ],
) -> ValidateDataSchemaResult:
    """Validate a data schema (JSON Schema) against the Fairspec Data Schema specification. Returns validation report with errors if any."""
    result = validate_data_schema(source)
    return ValidateDataSchemaResult(
        report=Report(valid=result.valid, errors=result.errors)
    )
