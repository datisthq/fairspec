from __future__ import annotations

from typing import Union

from fairspec_metadata import (
    CsvFileDialect,
    JsonFileDialect,
    JsonlFileDialect,
    OdsFileDialect,
    TsvFileDialect,
    XlsxFileDialect,
)

FileDialectWithHeaderAndCommentRows = Union[
    CsvFileDialect,
    JsonFileDialect,
    JsonlFileDialect,
    OdsFileDialect,
    TsvFileDialect,
    XlsxFileDialect,
]
