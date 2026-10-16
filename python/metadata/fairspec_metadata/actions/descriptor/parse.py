from __future__ import annotations

import json

from fairspec_metadata.models.descriptor import Descriptor


class Error(Exception):
    pass


def parse_descriptor(text: str) -> Descriptor:
    value = json.loads(text)
    if not isinstance(value, dict):
        raise Error(f"Invalid descriptor: {text}")
    return value
