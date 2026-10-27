from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_metadata.actions.frictionless.mapping import FrictionlessRecorder
    from fairspec_metadata.models.descriptor import Descriptor
    from fairspec_metadata.models.frictionless.dialect import FrictionlessDialect

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
    dialect: FrictionlessDialect | None,
    *,
    format: str | None = None,
    record: FrictionlessRecorder | None = None,
) -> Descriptor | None:
    """Convert a Frictionless Table Dialect to a Fairspec File Dialect descriptor

    Returns None for a format Fairspec does not read as a table; without a format, a
    standalone dialect takes the one its keys belong to
    """
    fairspec_format = (
        _infer_format(dialect)
        if format is None and dialect is not None
        else FORMATS.get((format or "").lower())
    )
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
            _record_keys(file_dialect, record)
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

    _record_keys(file_dialect, record)
    return file_dialect


DIALECT_KEYS = {
    "delimiter": "delimiter",
    "lineTerminator": "lineTerminator",
    "quoteChar": "quoteChar",
    "nullSequence": "nullSequence",
    "headerJoin": "headerJoin",
    "commentRows": "commentRows",
    "commentPrefix": "commentChar",
    "jsonPointer": "property",
    "rowType": "itemType",
    "columnNames": "itemKeys",
    "sheetName": "sheetName",
    "sheetNumber": "sheetNumber",
    "tableName": "table",
}


def _record_keys(file_dialect: Descriptor, record: FrictionlessRecorder | None) -> None:
    if record is None:
        return
    for key in file_dialect:
        if key == "headerRows":
            is_header = file_dialect["headerRows"] is False
            record("/headerRows", "/header" if is_header else "/headerRows")
        elif key in DIALECT_KEYS:
            record(f"/{key}", f"/{DIALECT_KEYS[key]}")


def _infer_format(dialect: FrictionlessDialect) -> str:
    if dialect.property is not None or dialect.itemType or dialect.itemKeys is not None:
        return "json"
    if dialect.sheetName is not None or dialect.sheetNumber is not None:
        return "xlsx"
    if dialect.table is not None:
        return "sqlite"
    return "csv"
