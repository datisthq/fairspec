from __future__ import annotations

import json

from .inspect import inspect_json_schema
from fairspec_metadata.models.descriptor import Descriptor
from fairspec_metadata.models.json_schema import JsonSchema


class Error(Exception):
    pass


def assert_json_schema(descriptor: Descriptor) -> JsonSchema:
    errors = inspect_json_schema(descriptor)

    if errors:
        preview = json.dumps(descriptor)[:100]
        raise Error(f'JsonSchema "{preview}" is not valid')

    return descriptor
