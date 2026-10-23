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

    paths = [path, *_get_all_of_paths(json_schema)]

    if type == ProfileType.data_schema:
        if path == "https://json-schema.org/draft/2020-12/schema":
            return json_schema

    for p in paths:
        if regex.search(p):
            return json_schema

    raise Error(f"Profile at path {path} is not a valid {type.value} profile")


def _get_all_of_paths(json_schema: JsonSchema) -> list[str]:
    paths: list[str] = []
    for item in json_schema.get("allOf") or []:
        if isinstance(item, str):
            paths.append(item)
        elif isinstance(item, dict) and isinstance(item.get("$ref"), str):
            paths.append(item["$ref"])
    return paths
