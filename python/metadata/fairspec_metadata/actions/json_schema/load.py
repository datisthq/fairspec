from __future__ import annotations

from functools import lru_cache

from fairspec_metadata.actions.descriptor.load import load_descriptor
from .assert_ import assert_json_schema
from fairspec_metadata.models.json_schema import JsonSchema


@lru_cache(maxsize=100)
def load_json_schema(
    path: str,
    *,
    only_remote: bool = False,
) -> JsonSchema:
    from fairspec_metadata.actions.profile.registry import profile_registry

    for item in profile_registry:
        if item.path == path:
            return item.profile

    descriptor = load_descriptor(path, only_remote=only_remote)
    return assert_json_schema(descriptor)
