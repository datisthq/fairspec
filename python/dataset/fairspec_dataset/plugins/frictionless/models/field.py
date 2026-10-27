from __future__ import annotations

from pydantic import JsonValue

from .base import FrictionlessModel


class FrictionlessConstraints(FrictionlessModel):
    required: bool | None = None
    unique: bool | None = None
    minimum: JsonValue = None
    maximum: JsonValue = None
    exclusiveMinimum: JsonValue = None
    exclusiveMaximum: JsonValue = None
    minLength: int | None = None
    maxLength: int | None = None
    pattern: str | None = None
    enum: list[JsonValue] | None = None
    jsonSchema: dict[str, JsonValue] | None = None


class FrictionlessMissingValueItem(FrictionlessModel):
    value: str
    label: str


FrictionlessMissingValue = str | FrictionlessMissingValueItem


class FrictionlessField(FrictionlessModel):
    name: str
    type: str | None = None
    format: str | None = None
    title: str | None = None
    description: str | None = None
    example: JsonValue = None
    rdfType: str | None = None
    constraints: FrictionlessConstraints | None = None
    missingValues: list[FrictionlessMissingValue] | None = None
    trueValues: list[str] | None = None
    falseValues: list[str] | None = None
    decimalChar: str | None = None
    groupChar: str | None = None
    bareNumber: bool | None = None
    categories: list[JsonValue] | None = None
    categoriesOrdered: bool | None = None
    delimiter: str | None = None
    itemType: str | None = None
