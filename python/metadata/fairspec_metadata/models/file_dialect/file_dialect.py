from __future__ import annotations

from typing import Annotated, Union

from pydantic import Field

from .arrow import ArrowFileDialect
from .csv import CsvFileDialect
from .json import JsonFileDialect
from .jsonl import JsonlFileDialect
from .ods import OdsFileDialect
from .parquet import ParquetFileDialect
from .sqlite import SqliteFileDialect
from .tsv import TsvFileDialect
from .xlsx import XlsxFileDialect

FileDialect = Annotated[
    Union[
        CsvFileDialect,
        TsvFileDialect,
        JsonFileDialect,
        JsonlFileDialect,
        XlsxFileDialect,
        OdsFileDialect,
        SqliteFileDialect,
        ParquetFileDialect,
        ArrowFileDialect,
    ],
    Field(discriminator="format"),
]
