from __future__ import annotations

import polars as pl

from fairspec_metadata import MetadataError, TableSchema


def inspect_table_schema_patterns(table_schema: TableSchema) -> list[MetadataError]:
    """Report each column pattern the table engine cannot compile, such as one using look-around"""
    errors: list[MetadataError] = []

    for name, property in (table_schema.properties or {}).items():
        pattern = getattr(property, "pattern", None)
        if not isinstance(pattern, str):
            continue

        try:
            pl.DataFrame({"value": [""]}).select(pl.col("value").str.contains(pattern))
        except pl.exceptions.PolarsError as error:
            reason = str(error).strip().split("\n")[-1]
            pointer = name.replace("~", "~0").replace("/", "~1")
            errors.append(
                MetadataError(
                    type="metadata",
                    message=f'pattern "{pattern}" cannot be compiled: {reason}',
                    jsonPointer=f"/properties/{pointer}/pattern",
                )
            )

    return errors
