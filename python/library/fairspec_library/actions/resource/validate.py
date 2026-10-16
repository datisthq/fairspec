from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_dataset import validate_file
from fairspec_metadata import Report, create_report

from fairspec_library.actions.data.validate import validate_data
from fairspec_library.actions.table.validate import validate_table
from fairspec_library.models.table import ValidateTableOptions

if TYPE_CHECKING:
    from fairspec_metadata import FairspecError, Resource


def validate_resource(
    resource: Resource, **options: Unpack[ValidateTableOptions]
) -> Report:
    errors: list[FairspecError] = []

    file_report = validate_file(resource, concurrency=options.get("concurrency"))
    errors.extend(file_report.errors)
    if not file_report.valid:
        return create_report(errors)

    data_report = validate_data(resource)
    errors.extend(data_report.errors)
    if not data_report.valid:
        return create_report(errors)

    table_report = validate_table(resource, **options)
    errors.extend(table_report.errors)

    return create_report(errors)
