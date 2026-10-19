from __future__ import annotations

from typing import TYPE_CHECKING

import polars as pl

from fairspec_table.helpers.file_dialect import get_header_rows, join_header_labels
from fairspec_table.settings import QUERY_ENGINE, NUMBER_COLUMN_NAME

if TYPE_CHECKING:
    from fairspec_table.models import FileDialectWithHeaderAndCommentRows, Table


# The table is read without a header, so its header rows are still data rows;
# with columnNames the first header row was read as the header and is gone
def join_header_rows(
    table: Table,
    dialect: FileDialectWithHeaderAndCommentRows,
) -> Table:
    header_rows = get_header_rows(dialect)
    if len(header_rows) < 2:
        return table

    skipped_rows = header_rows[0] - 1
    if dialect.columnNames:
        return _drop_rows(table, header_rows, skipped_rows + 1)

    header_frame: pl.DataFrame = table.head(header_rows[-1] - skipped_rows).collect(  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        engine=QUERY_ENGINE
    )
    header = [
        list(header_frame.row(number - skipped_rows - 1))
        if number - skipped_rows - 1 < header_frame.height
        else []
        for number in header_rows
    ]
    header_join = dialect.headerJoin if dialect.headerJoin is not None else " "
    labels = join_header_labels(header, header_join)

    names = table.collect_schema().names()
    mapping = {
        name: labels[index] if index < len(labels) and labels[index] else name
        for index, name in enumerate(names)
    }

    return _drop_rows(table, header_rows, skipped_rows).rename(mapping)


def skip_comment_rows(
    table: Table,
    dialect: FileDialectWithHeaderAndCommentRows,
) -> Table:
    header_rows = get_header_rows(dialect)
    comment_offset = header_rows[-1] if header_rows else 0
    if not dialect.commentRows:
        return table

    return _drop_rows(table, dialect.commentRows, comment_offset)


def _drop_rows(table: Table, numbers: list[int], offset: int) -> Table:
    return (
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .filter(pl.col(NUMBER_COLUMN_NAME).add(offset).is_in(numbers).not_())
        .drop(NUMBER_COLUMN_NAME)
    )
