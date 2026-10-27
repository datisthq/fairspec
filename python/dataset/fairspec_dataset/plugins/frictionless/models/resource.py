from __future__ import annotations

from pydantic import Field, JsonValue

from .base import FrictionlessModel
from .dialect import FrictionlessDialect
from .license import FrictionlessLicense
from .schema import FrictionlessSchema


class FrictionlessResource(FrictionlessModel):
    name: str | None = None
    path: str | list[str] | None = None
    data: JsonValue = None
    title: str | None = None
    description: str | None = None
    format: str | None = None
    mediatype: str | None = None
    encoding: str | None = None
    bytes: int | None = None
    hash: str | None = None
    licenses: list[FrictionlessLicense] | None = None
    dialect: str | FrictionlessDialect | None = None
    schema_: str | FrictionlessSchema | None = Field(default=None, alias="schema")
