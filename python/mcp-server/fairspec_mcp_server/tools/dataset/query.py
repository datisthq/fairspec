import json
from typing import Annotated

import polars as pl
from fairspec_library import Dataset, query_dataset
from pydantic import Field, JsonValue

from ...helpers.limit import DEFAULT_LIMIT, MAX_LIMIT, limit_items
from ...models.tool import ToolResult


class QueryDatasetResult(ToolResult):
    records: list[dict[str, JsonValue]]
    truncated: Annotated[bool, Field(description="Whether records were left out")]


def query_dataset_tool(
    source: Annotated[
        Dataset | str,
        Field(description="The dataset to query (dataset object or file path)"),
    ],
    query: Annotated[
        str, Field(description="SQL query to execute (use resource names as table names)")
    ],
    limit: Annotated[
        int,
        Field(
            ge=1,
            le=MAX_LIMIT,
            description=f"Maximum number of records to return (default {DEFAULT_LIMIT})",
        ),
    ] = DEFAULT_LIMIT,
) -> QueryDatasetResult:
    """Query a dataset using SQL. Each table resource is available under its name, so a query can join several resources. Returns the results as an array of records. Returns at most `limit` records; `truncated` is true when more were left out."""
    frame = query_dataset(source, query).head(limit + 1).collect()
    assert isinstance(frame, pl.DataFrame)
    records, truncated = limit_items(json.loads(frame.write_json()), limit=limit)
    return QueryDatasetResult(records=records, truncated=truncated)
