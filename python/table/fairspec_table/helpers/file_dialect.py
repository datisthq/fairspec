from __future__ import annotations

from fairspec_table.models.file_dialect import FileDialectWithHeaderAndCommentRows


def get_header_rows(
    file_dialect: FileDialectWithHeaderAndCommentRows | None = None,
) -> list[int]:
    if file_dialect is None or file_dialect.headerRows is None:
        return [1]
    if file_dialect.headerRows is False:
        return []
    return file_dialect.headerRows
