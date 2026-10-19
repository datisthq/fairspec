from __future__ import annotations

from fairspec_table.models.data import DataRow
from fairspec_table.models.file_dialect import FileDialectWithHeaderAndCommentRows


def get_header_rows(
    file_dialect: FileDialectWithHeaderAndCommentRows | None = None,
) -> list[int]:
    if file_dialect is None or file_dialect.headerRows is None:
        return [1]
    if file_dialect.headerRows is False:
        return []
    return file_dialect.headerRows


# A label carries forward across the empty cells to its right, so a group label
# spanning several columns prefixes each of them
def join_header_labels(header: list[DataRow], header_join: str) -> list[str]:
    width = max((len(row) for row in header), default=0)
    rows = [_fill_header_row(row, width) for row in header]

    return [
        header_join.join(row[index] for row in rows if row[index])
        for index in range(width)
    ]


def _fill_header_row(row: DataRow, width: int) -> list[str]:
    labels: list[str] = []

    label = ""
    for index in range(width):
        cell = row[index] if index < len(row) else None
        if cell is not None and str(cell) != "":
            label = str(cell)
        labels.append(label)

    return labels
