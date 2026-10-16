from __future__ import annotations

from typing import TYPE_CHECKING

import polars as pl

from fairspec_table.helpers.file_dialect import get_header_rows
from fairspec_table.settings import QUERY_ENGINE, NUMBER_COLUMN_NAME

if TYPE_CHECKING:
    from fairspec_table.models import FileDialectWithHeaderAndCommentRows, Table


def join_header_rows(
    table: Table,
    dialect: FileDialectWithHeaderAndCommentRows,
) -> Table:
    header_rows = get_header_rows(dialect)
    header_offset = header_rows[0] if header_rows else 0
    header_join = dialect.headerJoin if dialect.headerJoin is not None else " "
    if len(header_rows) < 2:
        return table

    labels = table.collect_schema().names()

    extra_labels_frame: pl.DataFrame = (  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .filter(pl.col(NUMBER_COLUMN_NAME).add(header_offset).is_in(header_rows))
        .select(*[pl.col(name).str.join(header_join) for name in labels])
        .collect(engine=QUERY_ENGINE)
    )
    extra_labels = extra_labels_frame.row(0)

    mapping = {
        label: header_join.join([label, extra_labels[index]])
        for index, label in enumerate(labels)
    }

    return (
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .filter(pl.col(NUMBER_COLUMN_NAME).add(header_offset).is_in(header_rows).not_())
        .rename(mapping)
        .drop(NUMBER_COLUMN_NAME)
    )


def skip_comment_rows(
    table: Table,
    dialect: FileDialectWithHeaderAndCommentRows,
) -> Table:
    header_rows = get_header_rows(dialect)
    comment_offset = header_rows[-1] if header_rows else 0
    if not dialect.commentRows:
        return table

    return (
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .filter(
            pl.col(NUMBER_COLUMN_NAME)
            .add(comment_offset)
            .is_in(dialect.commentRows)
            .not_()
        )
        .drop(NUMBER_COLUMN_NAME)
    )
