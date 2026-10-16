from __future__ import annotations

from dataclasses import dataclass
from typing import Literal, TypedDict

from fairspec_dataset import ConcurrencyOptions
from fairspec_metadata import TableSchema

from .column import PolarsColumn


@dataclass
class PolarsSchema:
    columns: list[PolarsColumn]


@dataclass
class SchemaMapping:
    source: PolarsSchema
    target: TableSchema


class TableSchemaOptions(TypedDict, total=False):
    columnNames: list[str]
    columnTypes: dict[str, str]
    missingValues: list[str]
    decimalChar: str
    groupChar: str
    trueValues: list[str]
    falseValues: list[str]
    datetimeFormat: str
    dateFormat: str
    timeFormat: str
    arrayType: Literal["array", "list"]
    listDelimiter: str
    listItemType: Literal[
        "string",
        "number",
        "boolean",
        "date",
        "date-time",
        "integer",
        "time",
    ]


class InferTableSchemaOptions(TableSchemaOptions, ConcurrencyOptions, total=False):
    sampleRows: int
    confidence: float
    commaDecimal: bool
    monthFirst: bool
    keepStrings: bool
