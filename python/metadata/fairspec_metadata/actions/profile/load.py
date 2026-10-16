from __future__ import annotations

from fairspec_metadata.actions.json_schema.load import load_json_schema
from .assert_ import assert_profile
from fairspec_metadata.models.profile import Profile, ProfileType


def load_profile(path: str, *, profile_type: ProfileType) -> Profile:
    json_schema = load_json_schema(path)

    return assert_profile(json_schema, path=path, type=profile_type)
