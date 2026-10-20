from __future__ import annotations

from collections.abc import Mapping

import polars as pl

from fairspec_table.models import Table


def query_table(table: Table, query: str) -> Table:
    """Run a SQL query against a table, exposed as `self`."""
    return query_tables({"self": table}, query)


def query_tables(tables: Mapping[str, Table], query: str) -> Table:
    """Run a SQL query against several tables, each exposed under its key."""
    context = pl.SQLContext(dict(tables))
    return context.execute(query)
