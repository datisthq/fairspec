from __future__ import annotations

from .base import FairspecModel

from .json_schema import JsonSchema

DataSchema = JsonSchema


class RenderDataSchemaOptions(FairspecModel):
    format: str
