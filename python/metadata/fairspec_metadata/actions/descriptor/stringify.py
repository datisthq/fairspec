from __future__ import annotations

import json

from fairspec_metadata.models.descriptor import Descriptor


def stringify_descriptor(descriptor: Descriptor) -> str:
    return json.dumps(descriptor, indent=2)
