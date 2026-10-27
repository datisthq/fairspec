from __future__ import annotations

import re
from typing import TYPE_CHECKING

from fairspec_metadata.actions.file_dialect.to_frictionless import (
    convert_file_dialect_to_frictionless,
)
from fairspec_metadata.actions.resource.infer import infer_resource_name
from fairspec_metadata.actions.rights.to_frictionless import (
    convert_rights_to_frictionless,
)
from fairspec_metadata.actions.table_schema.to_frictionless import (
    convert_table_schema_to_frictionless,
)
from fairspec_metadata.models.resource import Resource

if TYPE_CHECKING:
    from fairspec_metadata.models.descriptor import Descriptor


def convert_resource_to_frictionless(
    resource: Descriptor, *, resource_number: int
) -> Descriptor:
    """Convert a denormalized Fairspec Resource to a Frictionless Data Resource descriptor

    Its `tableSchema` and `fileDialect` must already be objects, not paths
    """
    data = resource.get("data")
    name = resource.get("name")
    if name is None:
        name = infer_resource_name(
            Resource.model_construct(data=data), resource_number=resource_number
        )
    frictionless_resource: Descriptor = {"name": name}

    if isinstance(data, str) or _get_is_path_list(data):
        frictionless_resource["path"] = data
    elif data is not None:
        frictionless_resource["data"] = data

    table_schema = resource.get("tableSchema")
    if isinstance(table_schema, dict):
        frictionless_resource["type"] = "table"

    title = (resource.get("titles") or [{}])[0].get("title")
    if title:
        frictionless_resource["title"] = title

    description = (resource.get("descriptions") or [{}])[0].get("description")
    if description:
        frictionless_resource["description"] = description

    file_dialect = resource.get("fileDialect")
    if isinstance(file_dialect, dict):
        converted = convert_file_dialect_to_frictionless(file_dialect)
        if converted:
            format, dialect = converted
            frictionless_resource["format"] = format
            if dialect:
                frictionless_resource["dialect"] = dialect

    integrity = resource.get("integrity")
    if integrity:
        frictionless_resource["hash"] = f"{integrity['type']}:{integrity['hash']}"

    for size in resource.get("sizes") or []:
        match = re.fullmatch(r"([0-9]+) bytes", size)
        if match:
            frictionless_resource["bytes"] = int(match.group(1))
            break

    rights_list = resource.get("rightsList")
    if rights_list:
        frictionless_resource["licenses"] = convert_rights_to_frictionless(rights_list)

    if isinstance(table_schema, dict):
        frictionless_resource["schema"] = convert_table_schema_to_frictionless(
            table_schema
        )

    return frictionless_resource


def _get_is_path_list(data: object) -> bool:
    return (
        isinstance(data, list)
        and len(data) > 0
        and all(isinstance(item, str) for item in data)
    )
