from __future__ import annotations

from fairspec_table.helpers import get_header_rows
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

    for row in content:
        if _get_is_commented_row(row, file_dialect):
            continue

        records.append(
            {
                labels[index]: row[index]
                for index in range(len(labels))
                if index < len(row)
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
        if number - 1 < len(rows):
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
    skip_rows = header_rows[0] - 1 if header_rows else 0

    content: list[DataRow] = []
    for index, row in enumerate(rows):
        number = index + 1

        if number <= skip_rows:
            continue

        if number in header_rows:
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

    labels = [str(v) for v in header[0]]
    header_join = (
        file_dialect.headerJoin
        if file_dialect is not None and file_dialect.headerJoin is not None
        else " "
    )

    for row in header[1:]:
        for index, label in enumerate(row):
            prefix = labels[index] if index < len(labels) else ""
            labels[index] = header_join.join(
                str(part) for part in [prefix, label] if part
            )

    return labels


def _get_is_commented_row(
    row: list[object],
    file_dialect: FileDialectWithHeaderAndCommentRows | None = None,
) -> bool:
    if file_dialect is None or file_dialect.commentPrefix is None:
        return False

    if not row or not isinstance(row[0], str):
        return False

    return row[0].startswith(file_dialect.commentPrefix)
