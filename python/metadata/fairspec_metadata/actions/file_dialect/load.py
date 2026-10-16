from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata.actions.descriptor.load import load_descriptor

from .assert_ import assert_file_dialect

if TYPE_CHECKING:
    from fairspec_metadata.models.file_dialect.file_dialect import FileDialect


def load_file_dialect(path: str) -> FileDialect:
    descriptor = load_descriptor(path)
    return assert_file_dialect(descriptor)
