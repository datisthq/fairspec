from __future__ import annotations

from enum import StrEnum
from typing import Annotated, Union

from pydantic import Field

from .array import ArrayColumn, ArrayColumnProperty
from .base64 import Base64Column, Base64ColumnProperty
from .boolean import BooleanColumn, BooleanColumnProperty
from .categorical import (
    CategoricalColumn,
    IntegerCategoricalColumnProperty,
    StringCategoricalColumnProperty,
)
from .date import DateColumn, DateColumnProperty
from .date_time import DateTimeColumn, DateTimeColumnProperty
from .decimal import DecimalColumn, DecimalColumnProperty
from .duration import DurationColumn, DurationColumnProperty
from .email import EmailColumn, EmailColumnProperty
from .geojson import GeojsonColumn, GeojsonColumnProperty
from .hex import HexColumn, HexColumnProperty
from .integer import IntegerColumn, IntegerColumnProperty
from .list import ListColumn, ListColumnProperty
from .number import NumberColumn, NumberColumnProperty
from .object import ObjectColumn, ObjectColumnProperty
from .string import StringColumn, StringColumnProperty
from .time import TimeColumn, TimeColumnProperty
from .topojson import TopojsonColumn, TopojsonColumnProperty
from .unknown import UnknownColumn, UnknownColumnProperty
from .url import UrlColumn, UrlColumnProperty
from .wkb import WkbColumn, WkbColumnProperty
from .wkt import WktColumn, WktColumnProperty

Column = Annotated[
    Union[
        ArrayColumn,
        Base64Column,
        BooleanColumn,
        CategoricalColumn,
        DateColumn,
        DateTimeColumn,
        DecimalColumn,
        DurationColumn,
        EmailColumn,
        GeojsonColumn,
        HexColumn,
        IntegerColumn,
        ListColumn,
        NumberColumn,
        ObjectColumn,
        StringColumn,
        TimeColumn,
        TopojsonColumn,
        UnknownColumn,
        UrlColumn,
        WkbColumn,
        WktColumn,
    ],
    Field(discriminator="type"),
]


class ColumnType(StrEnum):
    array = "array"
    base64 = "base64"
    boolean = "boolean"
    categorical = "categorical"
    date = "date"
    date_time = "date-time"
    decimal = "decimal"
    duration = "duration"
    email = "email"
    geojson = "geojson"
    hex = "hex"
    integer = "integer"
    list = "list"
    number = "number"
    object = "object"
    string = "string"
    time = "time"
    topojson = "topojson"
    unknown = "unknown"
    url = "url"
    wkb = "wkb"
    wkt = "wkt"


StringColumnPropertyWithFormat = Annotated[
    Union[
        ListColumnProperty,
        Base64ColumnProperty,
        HexColumnProperty,
        EmailColumnProperty,
        UrlColumnProperty,
        DateTimeColumnProperty,
        DateColumnProperty,
        TimeColumnProperty,
        DurationColumnProperty,
        WktColumnProperty,
        WkbColumnProperty,
        StringCategoricalColumnProperty,
        DecimalColumnProperty,
    ],
    Field(discriminator="format"),
]

ObjectColumnPropertyWithFormat = Annotated[
    Union[
        GeojsonColumnProperty,
        TopojsonColumnProperty,
    ],
    Field(discriminator="format"),
]

ColumnProperty = Union[
    BooleanColumnProperty,
    IntegerColumnProperty,
    IntegerCategoricalColumnProperty,
    NumberColumnProperty,
    StringColumnPropertyWithFormat,
    StringColumnProperty,
    ArrayColumnProperty,
    ObjectColumnPropertyWithFormat,
    ObjectColumnProperty,
    UnknownColumnProperty,
]
