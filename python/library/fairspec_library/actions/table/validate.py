from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_metadata import (
    Report,
    ResourceTypeError,
    create_report,
    resolve_table_schema,
)
from fairspec_table import inspect_table, inspect_table_schema_patterns

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
    checks = options.get("checks")

    table_schema = resolve_table_schema(resource.tableSchema)
    if not table_schema and not no_infer:
        table_schema = infer_table_schema(resource, **options)

    if not table_schema and not checks:
        return create_report()

    pattern_errors = inspect_table_schema_patterns(table_schema) if table_schema else []
    if pattern_errors:
        return create_report([*pattern_errors], max_errors=max_errors)

    table = load_table(resource, denormalized=True, **options)
    if table is None:
        return create_report(
            [ResourceTypeError(type="resource/type", expectedResourceType="table")]
        )

    errors = inspect_table(
        table,
        table_schema=table_schema,
        checks=checks,
        max_errors=max_errors,
        concurrency=options.get("concurrency"),
    )
    return create_report(list(errors))
