from __future__ import annotations

from pydantic import ConfigDict, Field

from fairspec_metadata.models.base import FairspecModel

from .schema import CkanSchema


class CkanResource(FairspecModel):
    model_config = ConfigDict(populate_by_name=True)

    id: str | None = None
    url: str | None = None
    name: str | None = None
    created: str | None = None
    description: str | None = None
    format: str | None = None
    hash: str | None = None
    last_modified: str | None = None
    metadata_modified: str | None = None
    mimetype: str | None = None
    size: int | None = None
    schema_: CkanSchema | None = Field(default=None, alias="schema")
