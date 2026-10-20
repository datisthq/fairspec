from typing import Annotated

from fairspec_library import (
    Report,
    Resource,
    ValidateTableOptions,
    infer_table_schema,
    resolve_table_schema,
    validate_table,
)
from pydantic import Field

from ...helpers.limit import DEFAULT_LIMIT, MAX_LIMIT, limit_report
from ...models.tool import ToolResult
from .._shared.schemas import MaxErrors, Truncated


class ValidateTableResult(ToolResult):
    report: Report
    truncated: Truncated


def validate_table_tool(
    resource: Annotated[Resource, Field(description="The table resource to validate")],
    options: Annotated[
        ValidateTableOptions | None, Field(description="Validation options")
    ] = None,
    maxErrors: MaxErrors = None,
) -> ValidateTableResult:
    """Validate a table resource against its schema. Returns a validation report with errors if any."""
    table_options: ValidateTableOptions = {**options} if options else {}
    max_errors = min(
        maxErrors or table_options.get("maxErrors") or DEFAULT_LIMIT, MAX_LIMIT
    )

    table_schema = resolve_table_schema(resource.tableSchema)
    if not table_schema and not table_options.get("noInfer"):
        table_schema = infer_table_schema(resource, **table_options)

    # The library splits maxErrors across columns, so ask every column for one error
    # more than the limit, or a single bad column would never show truncation
    column_count = max(len(table_schema.properties or {}) if table_schema else 0, 1)
    table_options["maxErrors"] = (max_errors + 1) * column_count

    resource = resource.model_copy(update={"tableSchema": table_schema})
    report = validate_table(resource, **table_options)
    report, truncated = limit_report(report, max_errors=max_errors)
    return ValidateTableResult(report=report, truncated=truncated)
