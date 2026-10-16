from __future__ import annotations

import re

from fairspec_metadata.models.json_schema import JsonSchema
from fairspec_metadata.models.profile import Profile, ProfileType


class Error(Exception):
    pass


def assert_profile(
    json_schema: JsonSchema,
    *,
    path: str,
    type: ProfileType,
) -> Profile:
    regex = re.compile(
        rf"^https://fairspec\.org/profiles/(\d+\.\d+\.\d+|latest)/{re.escape(type.value)}\.json$"
    )

    paths = [path]
    if isinstance(json_schema.get("allOf"), list):
        for ref in json_schema["allOf"]:
            if isinstance(ref, str):
                paths.append(ref)

    if type == ProfileType.data_schema:
        if path == "https://json-schema.org/draft/2020-12/schema":
            return json_schema

    for p in paths:
        if regex.search(p):
            return json_schema

    raise Error(f"Profile at path {path} is not a valid {type.value} profile")
