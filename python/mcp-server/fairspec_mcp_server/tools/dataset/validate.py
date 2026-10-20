from typing import Annotated

from fairspec_library import Dataset, Report, validate_dataset
from pydantic import Field

from ...models.tool import ToolResult


class ValidateDatasetResult(ToolResult):
    report: Report


def validate_dataset_tool(
    source: Annotated[
        Dataset | str,
        Field(description="The dataset to validate (dataset object or file path)"),
    ],
) -> ValidateDatasetResult:
    """Validate a dataset including all its resources and foreign keys. Returns a validation report with errors if any."""
    return ValidateDatasetResult(report=validate_dataset(source))
