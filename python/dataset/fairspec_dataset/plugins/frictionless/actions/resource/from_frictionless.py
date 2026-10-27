from __future__ import annotations

from typing import TYPE_CHECKING

from pydantic import JsonValue

from fairspec_metadata import get_file_extension

from fairspec_dataset.plugins.frictionless.helpers.name import (
    convert_name_from_frictionless,
)
from fairspec_dataset.plugins.frictionless.models.dialect import FrictionlessDialect
from fairspec_dataset.plugins.frictionless.models.schema import FrictionlessSchema
from fairspec_dataset.plugins.frictionless.actions.file_dialect.from_frictionless import (
    convert_file_dialect_from_frictionless,
)
from fairspec_dataset.plugins.frictionless.actions.rights.from_frictionless import (
    convert_rights_from_frictionless,
)
from fairspec_dataset.plugins.frictionless.actions.table_schema.from_frictionless import (
    convert_table_schema_from_frictionless,
)

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor

    from fairspec_dataset.plugins.frictionless.models.resource import (
        FrictionlessResource,
    )

HASH_TYPES = ("md5", "sha1", "sha256", "sha512")
MEDIA_TYPES = {
    "text/csv": "csv",
    "text/tab-separated-values": "tsv",
    "application/json": "json",
}


def convert_resource_from_frictionless(
    frictionless_resource: FrictionlessResource,
) -> Descriptor:
    """Convert a Frictionless Data Resource to a Fairspec Resource descriptor

    Its `schema` and `dialect` must already be objects, not paths
    """
    resource: Descriptor = {}
    path = frictionless_resource.path

    if path is not None:
        resource["data"] = path
    elif "data" in frictionless_resource.model_fields_set:
        resource["data"] = _convert_inline_data(frictionless_resource.data)

    if frictionless_resource.name:
        resource["name"] = convert_name_from_frictionless(frictionless_resource.name)

    if frictionless_resource.title:
        resource["titles"] = [{"title": frictionless_resource.title}]

    if frictionless_resource.description:
        resource["descriptions"] = [
            {
                "description": frictionless_resource.description,
                "descriptionType": "Abstract",
            }
        ]

    if frictionless_resource.licenses:
        resource["rightsList"] = convert_rights_from_frictionless(
            frictionless_resource.licenses
        )

    if frictionless_resource.bytes is not None:
        resource["sizes"] = [f"{frictionless_resource.bytes} bytes"]

    if frictionless_resource.hash:
        integrity = _convert_hash(frictionless_resource.hash)
        if integrity:
            resource["integrity"] = integrity

    if path is not None:
        first_path = path if isinstance(path, str) else next(iter(path), None)
        format = frictionless_resource.format
        if format is None:
            format = MEDIA_TYPES.get(frictionless_resource.mediatype or "")
        if format is None and first_path:
            format = get_file_extension(first_path)
        dialect = frictionless_resource.dialect
        file_dialect = convert_file_dialect_from_frictionless(
            dialect if isinstance(dialect, FrictionlessDialect) else None,
            format=format,
        )
        if file_dialect:
            resource["fileDialect"] = file_dialect

    schema = frictionless_resource.schema_
    if isinstance(schema, FrictionlessSchema):
        resource["tableSchema"] = convert_table_schema_from_frictionless(schema)

    return resource


def _convert_inline_data(data: JsonValue) -> JsonValue:
    if not isinstance(data, list):
        return data

    rows: list[list[JsonValue]] = []
    for row in data:
        if not isinstance(row, list):
            return data
        rows.append(row)

    header = rows[0] if rows else []
    return [
        {
            _stringify_name(name): row[index]
            if index < len(row) and row[index] is not None
            else None
            for index, name in enumerate(header)
        }
        for row in rows[1:]
    ]


def _stringify_name(name: JsonValue) -> str:
    if isinstance(name, bool):
        return "true" if name else "false"
    if name is None:
        return "null"
    return str(name)


def _convert_hash(hash: str) -> Descriptor | None:
    parts = hash.split(":") if ":" in hash else ["md5", hash]
    type, value = parts[0], parts[1]
    if not type or not value or type not in HASH_TYPES:
        return None
    return {"type": type, "hash": value}
