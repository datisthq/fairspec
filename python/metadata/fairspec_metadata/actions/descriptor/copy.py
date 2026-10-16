from __future__ import annotations

import copy

from fairspec_metadata.models.descriptor import Descriptor


def copy_descriptor(descriptor: Descriptor) -> Descriptor:
    return copy.deepcopy(descriptor)
