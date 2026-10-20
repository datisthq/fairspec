import json
from typing import Annotated

import polars as pl
from fairspec_library import Resource, load_table, query_table
from pydantic import Field, JsonValue

from ...models.tool import ToolResult


class QueryTableResult(ToolResult):
    records: list[dict[str, JsonValue]]


def query_table_tool(
    resource: Annotated[Resource, Field(description="The table resource to query")],
    query: Annotated[
        str, Field(description="SQL query to execute (use 'self' as table name)")
    ],
) -> QueryTableResult:
    """Query a table using SQL. Loads the table, executes the SQL query, and returns the results as an array of records."""
    table = load_table(resource)
    if table is None:
        raise ValueError("Could not load table")

    frame = query_table(table, query).collect()
    assert isinstance(frame, pl.DataFrame)
    return QueryTableResult(records=json.loads(frame.write_json()))
