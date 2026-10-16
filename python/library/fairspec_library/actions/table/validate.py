from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_metadata import Report, create_report, resolve_table_schema
from fairspec_table import inspect_table

from fairspec_library.actions.file_dialect.infer import infer_file_dialect
from fairspec_library.actions.table.load import load_table
from fairspec_library.actions.table_schema.infer import infer_table_schema
from fairspec_library.models.table import ValidateTableOptions

if TYPE_CHECKING:
    from fairspec_metadata import Resource


def validate_table(resource: Resource, **options: Unpack[ValidateTableOptions]) -> Report:
    resource = resource.model_copy(deep=True)

    if not resource.fileDialect:
        resource.fileDialect = infer_file_dialect(resource)

    no_infer = options.get("noInfer", False)
    max_errors = options.get("maxErrors", 1000)

    table_schema = resolve_table_schema(resource.tableSchema)
    if not table_schema and not no_infer:
        table_schema = infer_table_schema(resource, **options)

    table = load_table(resource, denormalized=True, **options)
    if table is None:
        return create_report()

    errors = inspect_table(
        table,
        table_schema=table_schema,
        max_errors=max_errors,
        concurrency=options.get("concurrency"),
    )
    return create_report(list(errors))
