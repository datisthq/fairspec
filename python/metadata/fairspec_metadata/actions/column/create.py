from __future__ import annotations

from typing import TYPE_CHECKING, cast

from fairspec_metadata.models.column.array import ArrayColumn, ArrayColumnProperty
from fairspec_metadata.models.column.base import BaseColumn, BaseColumnProperty
from fairspec_metadata.models.column.base64 import Base64Column, Base64ColumnProperty
from fairspec_metadata.models.column.boolean import (
    BooleanColumn,
    BooleanColumnProperty,
)
from fairspec_metadata.models.column.categorical import (
    CategoricalColumn,
    IntegerCategoricalColumnProperty,
    StringCategoricalColumnProperty,
)
from fairspec_metadata.models.column.column import ColumnType
from fairspec_metadata.models.column.date import DateColumn, DateColumnProperty
from fairspec_metadata.models.column.date_time import (
    DateTimeColumn,
    DateTimeColumnProperty,
)
from fairspec_metadata.models.column.decimal import DecimalColumn, DecimalColumnProperty
from fairspec_metadata.models.column.duration import (
    DurationColumn,
    DurationColumnProperty,
)
from fairspec_metadata.models.column.email import EmailColumn, EmailColumnProperty
from fairspec_metadata.models.column.geojson import GeojsonColumn, GeojsonColumnProperty
from fairspec_metadata.models.column.hex import HexColumn, HexColumnProperty
from fairspec_metadata.models.column.integer import IntegerColumn, IntegerColumnProperty
from fairspec_metadata.models.column.list import ListColumn, ListColumnProperty
from fairspec_metadata.models.column.number import NumberColumn, NumberColumnProperty
from fairspec_metadata.models.column.object import ObjectColumn, ObjectColumnProperty
from fairspec_metadata.models.column.string import StringColumn, StringColumnProperty
from fairspec_metadata.models.column.time import TimeColumn, TimeColumnProperty
from fairspec_metadata.models.column.topojson import (
    TopojsonColumn,
    TopojsonColumnProperty,
)
from fairspec_metadata.models.column.unknown import UnknownColumn, UnknownColumnProperty
from fairspec_metadata.models.column.url import UrlColumn, UrlColumnProperty
from fairspec_metadata.models.column.wkb import WkbColumn, WkbColumnProperty
from fairspec_metadata.models.column.wkt import WktColumn, WktColumnProperty

from .property import get_base_property_type, get_is_nullable_property_type

if TYPE_CHECKING:
    from fairspec_metadata.models.column.column import Column
    from fairspec_metadata.models.descriptor import Descriptor

_COLUMN_CLASS_MAP: dict[ColumnType, tuple[type[BaseColumn], type[BaseColumnProperty]]] = {
    ColumnType.array: (ArrayColumn, ArrayColumnProperty),
    ColumnType.base64: (Base64Column, Base64ColumnProperty),
    ColumnType.boolean: (BooleanColumn, BooleanColumnProperty),
    ColumnType.date: (DateColumn, DateColumnProperty),
    ColumnType.date_time: (DateTimeColumn, DateTimeColumnProperty),
    ColumnType.decimal: (DecimalColumn, DecimalColumnProperty),
    ColumnType.duration: (DurationColumn, DurationColumnProperty),
    ColumnType.email: (EmailColumn, EmailColumnProperty),
    ColumnType.geojson: (GeojsonColumn, GeojsonColumnProperty),
    ColumnType.hex: (HexColumn, HexColumnProperty),
    ColumnType.integer: (IntegerColumn, IntegerColumnProperty),
    ColumnType.list: (ListColumn, ListColumnProperty),
    ColumnType.number: (NumberColumn, NumberColumnProperty),
    ColumnType.object: (ObjectColumn, ObjectColumnProperty),
    ColumnType.string: (StringColumn, StringColumnProperty),
    ColumnType.time: (TimeColumn, TimeColumnProperty),
    ColumnType.topojson: (TopojsonColumn, TopojsonColumnProperty),
    ColumnType.unknown: (UnknownColumn, UnknownColumnProperty),
    ColumnType.url: (UrlColumn, UrlColumnProperty),
    ColumnType.wkb: (WkbColumn, WkbColumnProperty),
    ColumnType.wkt: (WktColumn, WktColumnProperty),
}

_CATEGORICAL_PROPERTY_MAP: dict[str | None, type[BaseColumnProperty]] = {
    "integer": IntegerCategoricalColumnProperty,
    "string": StringCategoricalColumnProperty,
}


def create_column_from_property(name: str, property: Descriptor) -> Column:
    base_type = get_base_property_type(property.get("type"))
    format = property.get("format")
    nullable = get_is_nullable_property_type(property.get("type")) or None
    column_type = _get_column_type(base_type, format)

    if column_type == ColumnType.categorical:
        cat_property_cls = _CATEGORICAL_PROPERTY_MAP.get(
            base_type, StringCategoricalColumnProperty
        )
        property_model = cat_property_cls.model_validate(property)
        # Upcast concrete subclass to Column union (type checker can't infer this)
        return cast(
            "Column",
            CategoricalColumn.model_validate(
                {
                    "type": column_type,
                    "name": name,
                    "nullable": nullable,
                    "property": property_model,
                }
            ),
        )

    column_cls, property_cls = _COLUMN_CLASS_MAP[column_type]
    property_model = property_cls.model_validate(property)
    # Upcast concrete subclass to Column union (type checker can't infer this)
    return cast(
        "Column",
        column_cls.model_validate(
            {
                "type": column_type,
                "name": name,
                "nullable": nullable,
                "property": property_model,
            }
        ),
    )


def _get_column_type(base_type: str | None, format: str | None) -> ColumnType:
    match base_type:
        case "boolean":
            return ColumnType.boolean
        case "integer":
            if format == "categorical":
                return ColumnType.categorical
            return ColumnType.integer
        case "number":
            return ColumnType.number
        case "string":
            match format:
                case "categorical":
                    return ColumnType.categorical
                case "decimal":
                    return ColumnType.decimal
                case "list":
                    return ColumnType.list
                case "base64":
                    return ColumnType.base64
                case "hex":
                    return ColumnType.hex
                case "email":
                    return ColumnType.email
                case "url":
                    return ColumnType.url
                case "date-time":
                    return ColumnType.date_time
                case "date":
                    return ColumnType.date
                case "time":
                    return ColumnType.time
                case "duration":
                    return ColumnType.duration
                case "wkt":
                    return ColumnType.wkt
                case "wkb":
                    return ColumnType.wkb
                case _:
                    return ColumnType.string
        case "array":
            return ColumnType.array
        case "object":
            match format:
                case "geojson":
                    return ColumnType.geojson
                case "topojson":
                    return ColumnType.topojson
                case _:
                    return ColumnType.object
        case _:
            return ColumnType.unknown
