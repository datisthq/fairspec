from typing import Annotated

from fairspec_library import Dataset, infer_dataset
from pydantic import Field

from ...models.tool import ToolResult


class InferDatasetResult(ToolResult):
    dataset: Dataset


def infer_dataset_tool(
    dataset: Annotated[Dataset, Field(description="The dataset to infer metadata for")],
) -> InferDatasetResult:
    """Infer metadata for all resources in a dataset. Analyzes each resource to determine schemas and dialects."""
    return InferDatasetResult(dataset=infer_dataset(dataset))
