from __future__ import annotations

import json
from typing import TYPE_CHECKING

from fairspec_dataset import load_file
from fairspec_metadata import (
    get_data_first_path,
    get_data_value,
    get_supported_file_dialect,
)

if TYPE_CHECKING:
    from fairspec_metadata import Resource


def load_data(resource: Resource) -> object | None:
    data_value = get_data_value(resource)
    if data_value:
        return data_value

    first_path = get_data_first_path(resource)
    if first_path:
        dialect = get_supported_file_dialect(resource, ["json"])
        if dialect:
            return json.loads(load_file(first_path))

    return None
