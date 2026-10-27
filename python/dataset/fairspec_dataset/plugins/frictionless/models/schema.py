from __future__ import annotations

from .base import FrictionlessModel
from .field import FrictionlessField, FrictionlessMissingValue

FieldNames = str | list[str]


class FrictionlessForeignKeyReference(FrictionlessModel):
    resource: str | None = None
    fields: FieldNames


class FrictionlessForeignKey(FrictionlessModel):
    fields: FieldNames
    reference: FrictionlessForeignKeyReference


class FrictionlessSchema(FrictionlessModel):
    fields: list[FrictionlessField]
    primaryKey: FieldNames | None = None
    uniqueKeys: list[list[str]] | None = None
    foreignKeys: list[FrictionlessForeignKey] | None = None
    missingValues: list[FrictionlessMissingValue] | None = None
