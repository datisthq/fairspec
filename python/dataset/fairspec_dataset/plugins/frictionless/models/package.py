from __future__ import annotations

from pydantic import Field

from .base import FrictionlessModel
from .license import FrictionlessLicense
from .resource import FrictionlessResource


class FrictionlessContributor(FrictionlessModel):
    title: str | None = None
    givenName: str | None = None
    familyName: str | None = None
    path: str | None = None
    email: str | None = None
    organization: str | None = None
    role: str | None = None
    roles: list[str] | None = None


class FrictionlessSource(FrictionlessModel):
    title: str | None = None
    path: str | None = None
    email: str | None = None


class FrictionlessPackage(FrictionlessModel):
    schema_: str | None = Field(default=None, alias="$schema")
    profile: str | None = None
    name: str | None = None
    id: str | None = None
    title: str | None = None
    description: str | None = None
    homepage: str | None = None
    version: str | None = None
    created: str | None = None
    keywords: list[str] | None = None
    licenses: list[FrictionlessLicense] | None = None
    contributors: list[FrictionlessContributor] | None = None
    sources: list[FrictionlessSource] | None = None
    resources: list[FrictionlessResource]
