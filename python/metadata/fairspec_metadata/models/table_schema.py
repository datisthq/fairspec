from __future__ import annotations

from typing import Union

from pydantic import ConfigDict, Field

from .base import FairspecModel

from .column.column import ColumnProperty
from .foreign_key import ForeignKey
from .unique_key import UniqueKey


class TableSchemaMissingValueItem(FairspecModel):
    value: str | int | float
    label: str


class TableSchema(FairspecModel):
    model_config = ConfigDict(populate_by_name=True)

    profile: str | None = Field(
        default=None,
        alias="$schema",
        description="Fairspec Schema profile url.",
    )
    title: str | None = Field(
        default=None,
        description="A human-readable title of the table schema",
    )
    description: str | None = Field(
        default=None,
        description="A human-readable description of the table schema",
    )
    required: list[str] | None = Field(
        default=None,
        description="An optional list of column names that must be present",
    )
    allRequired: bool | None = Field(
        default=None,
        description="An optional boolean indicating whether all columns are required",
    )
    properties: dict[str, ColumnProperty] | None = Field(
        default=None,
        description="An object defining the schema for table columns, where each key is a column name",
    )
    missingValues: list[Union[str, int, float, TableSchemaMissingValueItem]] | None = (
        Field(
            default=None,
            description="An optional list of values that represent missing or null data across all columns",
        )
    )
    primaryKey: list[str] | None = Field(
        default=None,
        min_length=1,
        description="An optional array of column names that form the table's primary key",
    )
    uniqueKeys: list[UniqueKey] | None = Field(
        default=None,
        min_length=1,
        description="An optional array of unique key constraints",
    )
    foreignKeys: list[ForeignKey] | None = Field(
        default=None,
        min_length=1,
        description="An optional array of foreign key constraints",
    )


class RenderTableSchemaOptions(FairspecModel):
    format: str


class ConvertTableSchemaToOptions(FairspecModel):
    format: str


class ConvertTableSchemaFromOptions(FairspecModel):
    format: str
