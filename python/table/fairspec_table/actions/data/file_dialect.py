from __future__ import annotations

from fairspec_table.helpers.file_dialect import (
    assert_unique_column_names,
    get_header_rows,
    join_header_labels,
)
from fairspec_table.models import (
    DataRecord,
    DataRow,
    FileDialectWithHeaderAndCommentRows,
)


def get_records_from_rows(
    rows: list[DataRow],
    file_dialect: FileDialectWithHeaderAndCommentRows | None = None,
) -> list[DataRecord]:
    records: list[DataRecord] = []

    header = _get_header_from_rows(rows, file_dialect)
    content = _get_content_from_rows(rows, file_dialect)

    labels = _get_labels_from_header(header, file_dialect)
    if labels is None:
        return records
    column_names = file_dialect.columnNames if file_dialect is not None else None
    if column_names is None:
        assert_unique_column_names(labels)

    for row in content:
        if _get_is_commented_row(row, file_dialect):
            continue

        if column_names is not None and len(row) > len(labels):
            raise Exception(
                f"A row has {len(row)} cells but columnNames has {len(labels)}"
            )

        records.append(
            {
                label: row[index] if index < len(row) else None
                for index, label in enumerate(labels)
            }
        )

    return records


def _get_header_from_rows(
    rows: list[DataRow],
    file_dialect: FileDialectWithHeaderAndCommentRows | None = None,
) -> list[DataRow]:
    if file_dialect is not None and file_dialect.columnNames is not None:
        return [list(file_dialect.columnNames)]

    header_rows = get_header_rows(file_dialect)

    if not header_rows:
        length = max((len(row) for row in rows), default=0)
        labels = [f"column{idx + 1}" for idx in range(length)]
        return [labels]

    header: list[DataRow] = []
    for number in header_rows:
        if number - 1 >= len(rows):
            raise Exception(f"Header row {number} is past the end of the data")
        header.append(rows[number - 1])

    return header


def _get_content_from_rows(
    rows: list[DataRow],
    file_dialect: FileDialectWithHeaderAndCommentRows | None = None,
) -> list[DataRow]:
    header_rows = get_header_rows(file_dialect)
    comment_rows = (
        file_dialect.commentRows
        if file_dialect is not None and file_dialect.commentRows is not None
        else []
    )
    skip_rows = max([0, *header_rows])

    content: list[DataRow] = []
    for index, row in enumerate(rows):
        number = index + 1

        if number <= skip_rows:
            continue

        if number in comment_rows:
            continue

        if _get_is_commented_row(row, file_dialect):
            continue

        content.append(row)

    return content


def _get_labels_from_header(
    header: list[DataRow],
    file_dialect: FileDialectWithHeaderAndCommentRows | None = None,
) -> list[str] | None:
    if not header:
        return None

    if len(header) == 1:
        return ["" if cell is None else str(cell) for cell in header[0]]

    header_join = (
        file_dialect.headerJoin
        if file_dialect is not None and file_dialect.headerJoin is not None
        else " "
    )
    return join_header_labels(header, header_join)


def _get_is_commented_row(
    row: list[object],
    file_dialect: FileDialectWithHeaderAndCommentRows | None = None,
) -> bool:
    if file_dialect is None or file_dialect.commentPrefix is None:
        return False

    if not row or not isinstance(row[0], str):
        return False

    return row[0].startswith(file_dialect.commentPrefix)
