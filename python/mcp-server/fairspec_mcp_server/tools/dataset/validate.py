from typing import Annotated

from fairspec_library import Dataset, Report, validate_dataset
from pydantic import Field

from ...helpers.limit import limit_report
from ...models.tool import ToolResult
from .._shared.schemas import MaxErrors, Truncated


class ValidateDatasetResult(ToolResult):
    report: Report
    truncated: Truncated


def validate_dataset_tool(
    source: Annotated[
        Dataset | str,
        Field(description="The dataset to validate (dataset object or file path)"),
    ],
    maxErrors: MaxErrors = None,
) -> ValidateDatasetResult:
    """Validate a dataset including all its resources and foreign keys. Returns a validation report with errors if any."""
    report, truncated = limit_report(validate_dataset(source), max_errors=maxErrors)
    return ValidateDatasetResult(report=report, truncated=truncated)
