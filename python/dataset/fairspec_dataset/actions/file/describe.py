from __future__ import annotations

from fairspec_metadata import Resource

from fairspec_dataset.models.file import FileDescription

from .infer import infer_bytes, infer_integrity, infer_textual
from .prefetch import prefetch_file


def describe_file(
    path: str,
    *,
    hash_type: str = "sha256",
) -> FileDescription:
    local_path = prefetch_file(path)
    resource = Resource(data=local_path)

    return FileDescription(
        bytes=infer_bytes(resource),
        textual=infer_textual(resource),
        integrity=infer_integrity(resource, hash_type=hash_type),
    )
