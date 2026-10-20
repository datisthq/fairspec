import json
from typing import Annotated

import polars as pl
from fairspec_library import Dataset, query_dataset
from pydantic import Field, JsonValue

from ...models.tool import ToolResult


class QueryDatasetResult(ToolResult):
    records: list[dict[str, JsonValue]]


def query_dataset_tool(
    source: Annotated[
        Dataset | str,
        Field(description="The dataset to query (dataset object or file path)"),
    ],
    query: Annotated[
        str, Field(description="SQL query to execute (use resource names as table names)")
    ],
) -> QueryDatasetResult:
    """Query a dataset using SQL. Each table resource is available under its name, so a query can join several resources. Returns the results as an array of records."""
    frame = query_dataset(source, query).collect()
    assert isinstance(frame, pl.DataFrame)
    return QueryDatasetResult(records=json.loads(frame.write_json()))
