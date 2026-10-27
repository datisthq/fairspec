from __future__ import annotations

from typing import TYPE_CHECKING

import polars as pl

from fairspec_table.helpers.file_dialect import (
    assert_unique_column_names,
    get_header_rows,
    join_header_labels,
)
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

    # Every row up to the last header row is consumed by the header, listed or not
    skipped_rows = header_rows[0] - 1
    last_header_row = header_rows[-1]
    header_height = last_header_row - skipped_rows - (1 if dialect.columnNames else 0)
    header_frame: pl.DataFrame = table.head(header_height).collect(  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        engine=QUERY_ENGINE
    )
    if header_frame.height < header_height:
        raise Exception(f"Header row {last_header_row} is past the end of the file")
    if dialect.columnNames:
        return _drop_first_rows(table, header_height)

    header = [list(header_frame.row(number - skipped_rows - 1)) for number in header_rows]
    header_join = dialect.headerJoin if dialect.headerJoin is not None else " "
    labels = join_header_labels(header, header_join)

    names = table.collect_schema().names()
    new_names = [
        labels[index] if index < len(labels) else name for index, name in enumerate(names)
    ]
    assert_unique_column_names(new_names)

    return _drop_first_rows(table, last_header_row - skipped_rows).rename(
        dict(zip(names, new_names, strict=True))
    )


def skip_comment_rows(
    table: Table,
    dialect: FileDialectWithHeaderAndCommentRows,
) -> Table:
    header_rows = get_header_rows(dialect)
    comment_offset = header_rows[-1] if header_rows else 0
    if not dialect.commentRows:
        return table

    return _drop_rows(table, dialect.commentRows, comment_offset)


def _drop_first_rows(table: Table, count: int) -> Table:
    return (
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .filter(pl.col(NUMBER_COLUMN_NAME) > count)
        .drop(NUMBER_COLUMN_NAME)
    )


def _drop_rows(table: Table, numbers: list[int], offset: int) -> Table:
    return (
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .filter(pl.col(NUMBER_COLUMN_NAME).add(offset).is_in(numbers).not_())
        .drop(NUMBER_COLUMN_NAME)
    )
