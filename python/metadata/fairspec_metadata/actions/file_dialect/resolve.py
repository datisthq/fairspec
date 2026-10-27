from __future__ import annotations

from typing import TYPE_CHECKING

from .assert_ import assert_file_dialect
from .load import load_file_dialect

if TYPE_CHECKING:
    from fairspec_metadata.models.file_dialect.file_dialect import FileDialect


def resolve_file_dialect(
    file_dialect: FileDialect | str | None = None,
) -> FileDialect | None:
    if file_dialect is None:
        return None

    # A dialect that is not valid never reads a file
    if not isinstance(file_dialect, str):
        assert_file_dialect(file_dialect.model_dump(by_alias=True, exclude_none=True))
        return file_dialect

    return load_file_dialect(file_dialect)
