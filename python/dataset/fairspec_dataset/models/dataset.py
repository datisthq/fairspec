from __future__ import annotations

from typing import Required, TypedDict

from fairspec_metadata.models.base import FairspecModel


class SaveDatasetOptions(TypedDict, total=False):
    target: Required[str]
    with_remote: bool


class SaveDatasetResult(FairspecModel):
    path: str | None = None
