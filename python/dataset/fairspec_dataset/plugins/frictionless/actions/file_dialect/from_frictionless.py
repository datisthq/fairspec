from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor

    from fairspec_dataset.plugins.frictionless.models.dialect import (
        FrictionlessDialect,
    )

FORMATS = {
    "csv": "csv",
    "tsv": "tsv",
    "json": "json",
    "jsonl": "jsonl",
    "ndjson": "jsonl",
    "xlsx": "xlsx",
    "ods": "ods",
    "parquet": "parquet",
    "arrow": "arrow",
    "feather": "arrow",
    "sqlite": "sqlite",
}


def convert_file_dialect_from_frictionless(
    dialect: FrictionlessDialect | None, *, format: str | None = None
) -> Descriptor | None:
    """Convert a Frictionless Table Dialect to a Fairspec File Dialect descriptor

    Returns None for a format Fairspec does not read as a table
    """
    fairspec_format = FORMATS.get((format or "").lower())
    if not fairspec_format:
        return None

    file_dialect: Descriptor = {"format": fairspec_format}
    if dialect is None:
        return file_dialect

    match fairspec_format:
        case "csv":
            if dialect.delimiter is not None:
                file_dialect["delimiter"] = dialect.delimiter
            if dialect.lineTerminator is not None:
                file_dialect["lineTerminator"] = dialect.lineTerminator
            if dialect.quoteChar is not None:
                file_dialect["quoteChar"] = dialect.quoteChar
            if dialect.nullSequence is not None:
                file_dialect["nullSequence"] = dialect.nullSequence
        case "tsv":
            if dialect.lineTerminator is not None:
                file_dialect["lineTerminator"] = dialect.lineTerminator
            if dialect.nullSequence is not None:
                file_dialect["nullSequence"] = dialect.nullSequence
        case "json":
            if dialect.property is not None:
                escaped = dialect.property.replace("~", "~0").replace("/", "~1")
                file_dialect["jsonPointer"] = f"/{escaped}"
            if dialect.itemType:
                file_dialect["rowType"] = dialect.itemType
        case "jsonl":
            if dialect.itemType:
                file_dialect["rowType"] = dialect.itemType
        case "xlsx" | "ods":
            if dialect.sheetName is not None:
                file_dialect["sheetName"] = dialect.sheetName
            if dialect.sheetNumber is not None:
                file_dialect["sheetNumber"] = dialect.sheetNumber
        case "sqlite":
            if dialect.table is not None:
                file_dialect["tableName"] = dialect.table
            return file_dialect
        case _:
            return file_dialect

    if dialect.header is False:
        file_dialect["headerRows"] = False
    elif dialect.headerRows is not None:
        file_dialect["headerRows"] = dialect.headerRows
    if dialect.headerJoin is not None:
        file_dialect["headerJoin"] = dialect.headerJoin
    if dialect.commentRows is not None:
        file_dialect["commentRows"] = dialect.commentRows
    if dialect.commentChar is not None:
        file_dialect["commentPrefix"] = dialect.commentChar
    if fairspec_format.startswith("json") and dialect.itemKeys is not None:
        file_dialect["columnNames"] = dialect.itemKeys

    return file_dialect
