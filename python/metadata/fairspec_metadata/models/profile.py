from __future__ import annotations

from enum import StrEnum

from .base import FairspecModel

from .json_schema import JsonSchema

Profile = JsonSchema


class ProfileType(StrEnum):
    catalog = "catalog"
    dataset = "dataset"
    file_dialect = "file-dialect"
    data_schema = "data-schema"
    table_schema = "table-schema"


class ProfileRegistryItem(FairspecModel):
    type: ProfileType
    path: str
    version: str
    profile: Profile


ProfileRegistry = list[ProfileRegistryItem]
