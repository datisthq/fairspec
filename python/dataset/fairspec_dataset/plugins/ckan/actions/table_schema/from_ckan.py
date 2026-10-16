from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import get_column_properties
from fairspec_metadata import ArrayColumn, ArrayColumnProperty
from fairspec_metadata import BooleanColumn, BooleanColumnProperty
from fairspec_metadata import DateColumn, DateColumnProperty
from fairspec_metadata import (
    DateTimeColumn,
    DateTimeColumnProperty,
)
from fairspec_metadata import IntegerColumn, IntegerColumnProperty
from fairspec_metadata import NumberColumn, NumberColumnProperty
from fairspec_metadata import ObjectColumn, ObjectColumnProperty
from fairspec_metadata import StringColumn, StringColumnProperty
from fairspec_metadata import TimeColumn, TimeColumnProperty
from fairspec_metadata import TableSchema

if TYPE_CHECKING:
    from fairspec_metadata import Column

    from fairspec_dataset.plugins.ckan.models.field import CkanField
    from fairspec_dataset.plugins.ckan.models.schema import CkanSchema


def convert_table_schema_from_ckan(ckan_schema: CkanSchema) -> TableSchema:
    columns: list[Column] = []

    for ckan_field in ckan_schema.fields or []:
        columns.append(_convert_column(ckan_field))

    return TableSchema(properties=get_column_properties(columns))


def _convert_column(ckan_field: CkanField) -> Column:
    info = ckan_field.info

    base_kwargs: dict = {}
    if info:
        if info.label:
            base_kwargs["title"] = info.label
        if info.notes:
            base_kwargs["description"] = info.notes

    column_type = (
        (info.type_override if info else None) or ckan_field.type or "text"
    ).lower()

    name = ckan_field.id or ""

    match column_type:
        case "text" | "string":
            return StringColumn(
                name=name,
                type="string",
                property=StringColumnProperty(type="string", **base_kwargs),
            )
        case "int" | "integer":
            return IntegerColumn(
                name=name,
                type="integer",
                property=IntegerColumnProperty(type="integer", **base_kwargs),
            )
        case "numeric" | "number" | "float":
            return NumberColumn(
                name=name,
                type="number",
                property=NumberColumnProperty(type="number", **base_kwargs),
            )
        case "bool" | "boolean":
            return BooleanColumn(
                name=name,
                type="boolean",
                property=BooleanColumnProperty(type="boolean", **base_kwargs),
            )
        case "date":
            return DateColumn(
                name=name,
                type="date",
                property=DateColumnProperty(type="string", format="date", **base_kwargs),
            )
        case "time":
            return TimeColumn(
                name=name,
                type="time",
                property=TimeColumnProperty(type="string", format="time", **base_kwargs),
            )
        case "timestamp" | "datetime":
            return DateTimeColumn(
                name=name,
                type="date-time",
                property=DateTimeColumnProperty(
                    type="string", format="date-time", **base_kwargs
                ),
            )
        case "json" | "object":
            return ObjectColumn(
                name=name,
                type="object",
                property=ObjectColumnProperty(type="object", **base_kwargs),
            )
        case "array":
            return ArrayColumn(
                name=name,
                type="array",
                property=ArrayColumnProperty(type="array", **base_kwargs),
            )
        case _:
            return StringColumn(
                name=name,
                type="string",
                property=StringColumnProperty(type="string", **base_kwargs),
            )
