from __future__ import annotations

from fairspec_metadata import (
    Resource,
    infer_resource_name,
    resolve_file_dialect,
)
from fairspec_dataset import infer_integrity, infer_textual

from fairspec_library.actions.data_schema.infer import infer_data_schema
from fairspec_library.actions.file_dialect.infer import infer_file_dialect
from fairspec_library.actions.table_schema.infer import infer_table_schema


def infer_resource(
    resource: Resource,
    *,
    resource_number: int | None = None,
    concurrency: int | None = None,
) -> Resource:
    resource = resource.model_copy(deep=True)

    if not resource.name:
        resource.name = infer_resource_name(resource, resource_number=resource_number)

    if not resource.fileDialect:
        resource.fileDialect = infer_file_dialect(resource)

    if resource.textual is None:
        resolved_dialect = resolve_file_dialect(resource.fileDialect)
        if resolved_dialect:
            resource.textual = infer_textual(resource)

    if not resource.integrity:
        resource.integrity = infer_integrity(resource, concurrency=concurrency)

    if not resource.dataSchema:
        resource.dataSchema = infer_data_schema(resource)

    if not resource.tableSchema:
        resource.tableSchema = infer_table_schema(resource)

    return resource
