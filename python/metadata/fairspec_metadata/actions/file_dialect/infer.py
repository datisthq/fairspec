from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata.actions.path.general import get_file_extension
from fairspec_metadata.actions.resource.data import get_data_first_path

if TYPE_CHECKING:
    from fairspec_metadata.models.resource import Resource

_EXTENSION_TO_FORMAT: dict[str, str] = {
    "csv": "csv",
    "tsv": "tsv",
    "json": "json",
    "jsonl": "jsonl",
    "ndjson": "jsonl",
    "xlsx": "xlsx",
    "ods": "ods",
    "parquet": "parquet",
    "arrow": "arrow",
    "feather": "arrow",
    "sqlite": "sqlite",
}


def infer_file_dialect_format(resource: Resource) -> str | None:
    path = get_data_first_path(resource)
    if not path:
        return None

    extension = get_file_extension(path)
    if not extension:
        return None

    return _EXTENSION_TO_FORMAT.get(extension)
