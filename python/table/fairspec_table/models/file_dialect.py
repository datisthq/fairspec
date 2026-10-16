from __future__ import annotations

from typing import Union

from fairspec_metadata import (
    CsvFileDialect,
    OdsFileDialect,
    TsvFileDialect,
    XlsxFileDialect,
)

FileDialectWithHeaderAndCommentRows = Union[
    CsvFileDialect, OdsFileDialect, TsvFileDialect, XlsxFileDialect
]
