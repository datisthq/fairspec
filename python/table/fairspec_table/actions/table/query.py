from __future__ import annotations

import polars as pl

from fairspec_table.models import Table


def query_table(table: Table, query: str) -> Table:
    context = pl.SQLContext({"self": table})
    return context.execute(query)
