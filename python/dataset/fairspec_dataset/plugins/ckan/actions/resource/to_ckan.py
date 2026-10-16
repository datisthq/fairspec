from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import TableSchema

from fairspec_dataset.plugins.ckan.actions.table_schema.to_ckan import (
    convert_table_schema_to_ckan,
)
from fairspec_dataset.plugins.ckan.models.resource import CkanResource

if TYPE_CHECKING:
    from fairspec_metadata import Resource


def convert_resource_to_ckan(resource: Resource) -> CkanResource:
    name = resource.name if resource.name else None

    fmt = None
    file_dialect = resource.fileDialect
    if not isinstance(file_dialect, str) and file_dialect is not None:
        fmt = file_dialect.format
        if fmt:
            fmt = fmt.upper()

    description = None
    descriptions = resource.descriptions or []
    if descriptions and descriptions[0].description:
        description = descriptions[0].description

    hash_val = None
    integrity = resource.integrity
    if integrity and integrity.hash:
        hash_val = integrity.hash

    created = None
    updated = None
    dates = resource.dates or []
    created_date = next((d for d in dates if d.dateType == "Created"), None)
    if created_date:
        created = created_date.date
    updated_date = next((d for d in dates if d.dateType == "Updated"), None)
    if updated_date:
        updated = updated_date.date

    schema = None
    table_schema = resource.tableSchema
    if isinstance(table_schema, TableSchema):
        schema = convert_table_schema_to_ckan(table_schema)

    return CkanResource(
        name=name,
        format=fmt,
        description=description,
        hash=hash_val,
        created=created,
        last_modified=updated,
        schema_=schema,
    )
