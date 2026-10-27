from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata.models.error.metadata import MetadataError

if TYPE_CHECKING:
    from fairspec_metadata.models.file_dialect.file_dialect import FileDialect


def inspect_file_dialect(
    file_dialect: FileDialect,
    *,
    root_json_pointer: str | None = None,
) -> list[MetadataError]:
    """Check what the profile cannot: a row listed both in headerRows and commentRows"""
    root = "" if root_json_pointer in (None, "/") else root_json_pointer
    errors: list[MetadataError] = []

    comment_rows = getattr(file_dialect, "commentRows", None)
    if not isinstance(comment_rows, list):
        return errors
    if getattr(file_dialect, "rowType", None) == "object":
        return errors

    declared = getattr(file_dialect, "headerRows", None)
    header_rows = (
        declared if isinstance(declared, list) else [] if declared is False else [1]
    )
    rows = [row for row in comment_rows if row in header_rows]
    if rows:
        listed = ", ".join(str(row) for row in rows)
        errors.append(
            MetadataError(
                type="metadata",
                message=f"row {listed} is listed both in headerRows and commentRows",
                jsonPointer=f"{root}/commentRows",
            )
        )

    return errors
