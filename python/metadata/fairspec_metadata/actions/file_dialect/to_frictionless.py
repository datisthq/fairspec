from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_metadata.models.descriptor import Descriptor

BUILTIN_FORMATS = (
    "csv",
    "tsv",
    "json",
    "jsonl",
    "xlsx",
    "ods",
    "parquet",
    "arrow",
    "sqlite",
)
COPIED_KEYS = (
    "delimiter",
    "lineTerminator",
    "quoteChar",
    "nullSequence",
    "headerJoin",
    "commentRows",
    "sheetName",
    "sheetNumber",
)


def convert_file_dialect_to_frictionless(
    file_dialect: Descriptor,
) -> tuple[str, Descriptor] | None:
    """Convert a Fairspec File Dialect to a Frictionless resource format and Table Dialect"""
    format = file_dialect.get("format")
    if format not in BUILTIN_FORMATS:
        return None

    dialect: Descriptor = {}

    for key, value in file_dialect.items():
        if value is None:
            continue
        if key in COPIED_KEYS:
            dialect[key] = value
        elif key == "headerRows":
            if value is False:
                dialect["header"] = False
            else:
                dialect["headerRows"] = value
        elif key == "commentPrefix":
            dialect["commentChar"] = value
        elif key == "jsonPointer":
            if isinstance(value, str) and value != "":
                dialect["property"] = value[1:].replace("~1", "/").replace("~0", "~")
        elif key == "rowType":
            dialect["itemType"] = value
        elif key == "columnNames":
            dialect["itemKeys"] = value
        elif key == "tableName":
            dialect["table"] = value

    return format, dialect
