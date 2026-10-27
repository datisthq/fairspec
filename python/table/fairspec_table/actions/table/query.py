from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass

import polars as pl

from fairspec_table.models import Table


def query_table(table: Table, query: str) -> Table:
    """Run a SQL query against a table, exposed as `self`."""
    return query_tables({"self": table}, query)


def query_tables(tables: Mapping[str, Table], query: str) -> Table:
    """Run a SQL query against several tables, each exposed under its key."""
    context = pl.SQLContext(dict(tables))
    return context.execute(set_nulls_last(query))


# A keyword that ends an ORDER BY list at the depth it opened
ORDER_END_KEYWORDS = frozenset(
    {
        "LIMIT",
        "OFFSET",
        "FETCH",
        "UNION",
        "EXCEPT",
        "INTERSECT",
        "ROWS",
        "RANGE",
        "GROUPS",
    }
)


def set_nulls_last(query: str) -> str:
    """Add `NULLS LAST` to every ORDER BY item that names no null order.

    Polars sorts nulls first in descending order, while the spec places them last
    in both directions.
    """
    positions: list[int] = []
    frames = [_OrderFrame()]
    index = 0
    size = len(query)

    while index < size:
        char = query[index]
        frame = frames[-1]

        if char in "'\"`":
            index = _skip_quoted(query, index)
            frame.mark(index, None)
        elif query.startswith("--", index):
            end = query.find("\n", index)
            index = size if end == -1 else end
        elif query.startswith("/*", index):
            end = query.find("*/", index + 2)
            index = size if end == -1 else end + 2
        elif char.isspace():
            index += 1
        elif char == "(":
            frames.append(_OrderFrame())
            index += 1
        elif char == ")":
            if len(frames) > 1:
                frames.pop().finish_item(positions)
                frames[-1].mark(index + 1, None)
            index += 1
        elif char in ",;":
            frame.finish_item(positions)
            if char == ";":
                frame.is_ordering = False
            frame.last_word = None
            index += 1
        elif char.isalnum() or char == "_":
            end = index
            while end < size and (query[end].isalnum() or query[end] in "_$"):
                end += 1
            word = query[index:end].upper()
            if frame.is_ordering and word in ORDER_END_KEYWORDS:
                frame.finish_item(positions)
                frame.is_ordering = False
            if word == "BY" and frame.last_word == "ORDER":
                frame.is_ordering = True
                frame.item_end = None
                frame.has_nulls = False
            else:
                frame.has_nulls = frame.has_nulls or word == "NULLS"
                frame.mark(end, word)
            index = end
        else:
            frame.mark(index + 1, None)
            index += 1

    for frame in frames:
        frame.finish_item(positions)

    for position in sorted(positions, reverse=True):
        query = f"{query[:position]} NULLS LAST{query[position:]}"
    return query


@dataclass
class _OrderFrame:
    is_ordering: bool = False
    has_nulls: bool = False
    item_end: int | None = None
    last_word: str | None = None

    def mark(self, end: int, word: str | None) -> None:
        if self.is_ordering:
            self.item_end = end
        self.last_word = word

    def finish_item(self, positions: list[int]) -> None:
        if self.is_ordering and self.item_end is not None and not self.has_nulls:
            positions.append(self.item_end)
        self.item_end = None
        self.has_nulls = False


def _skip_quoted(query: str, index: int) -> int:
    quote = query[index]
    index += 1
    while index < len(query):
        if query[index] == quote:
            if query.startswith(quote * 2, index):
                index += 2
                continue
            return index + 1
        index += 1
    return index
