import json
from typing import Annotated

import polars as pl
from fairspec_library import Resource, load_table, query_table
from pydantic import Field, JsonValue

from ...helpers.limit import DEFAULT_LIMIT, MAX_LIMIT, limit_items
from ...models.tool import ToolResult


class QueryTableResult(ToolResult):
    records: list[dict[str, JsonValue]]
    truncated: Annotated[bool, Field(description="Whether records were left out")]


def query_table_tool(
    resource: Annotated[Resource, Field(description="The table resource to query")],
    query: Annotated[
        str, Field(description="SQL query to execute (use 'self' as table name)")
    ],
    limit: Annotated[
        int,
        Field(
            ge=1,
            le=MAX_LIMIT,
            description=f"Maximum number of records to return (default {DEFAULT_LIMIT})",
        ),
    ] = DEFAULT_LIMIT,
) -> QueryTableResult:
    """Query a table using SQL. Loads the table, executes the SQL query, and returns the results as an array of records. Returns at most `limit` records; `truncated` is true when more were left out."""
    table = load_table(resource)
    if table is None:
        raise ValueError("Could not load table")

    frame = query_table(table, query).head(limit + 1).collect()
    assert isinstance(frame, pl.DataFrame)
    records, truncated = limit_items(json.loads(frame.write_json()), limit=limit)
    return QueryTableResult(records=records, truncated=truncated)
