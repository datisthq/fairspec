from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_metadata import (
    Dataset,
    Descriptor,
    FairspecException,
    Report,
    assert_dataset,
    create_report,
    infer_resource_name,
    load_dataset_descriptor,
    normalize_dataset,
)

from fairspec_dataset.helpers.concurrency import run_concurrent_tasks

from fairspec_library.actions.dataset.foreign_key import validate_dataset_foreign_keys
from fairspec_library.actions.resource.validate import validate_resource
from fairspec_library.models.table import ValidateTableOptions

if TYPE_CHECKING:
    from fairspec_metadata import FairspecError, Resource


def validate_dataset(
    source: Dataset | Descriptor | str, **options: Unpack[ValidateTableOptions]
) -> Report:
    if isinstance(source, str):
        try:
            descriptor = load_dataset_descriptor(source)
            source = Dataset.model_validate(descriptor)
        except FairspecException as exception:
            if exception.report:
                return exception.report
            return create_report()
    elif isinstance(source, dict):
        try:
            source = assert_dataset(source)
        except FairspecException as exception:
            if exception.report:
                return exception.report
            return create_report()

    dataset = normalize_dataset(source)
    errors = _validate_dataset_resources(dataset, **options)
    fk_report = validate_dataset_foreign_keys(dataset, **options)
    errors.extend(fk_report.errors)

    return create_report(errors)


def _validate_dataset_resources(
    dataset: Dataset, **options: Unpack[ValidateTableOptions]
) -> list[FairspecError]:
    resources = list(dataset.resources or [])

    for index, resource in enumerate(resources):
        if not resource.name:
            resource.name = infer_resource_name(resource, resource_number=index + 1)

    def validate(resource: Resource) -> list[FairspecError]:
        report = validate_resource(resource, **options)
        for error in report.errors:
            error.resourceName = resource.name
        return report.errors

    errors: list[FairspecError] = []
    for resource_errors in run_concurrent_tasks(
        validate, resources, concurrency=options.get("concurrency")
    ):
        errors.extend(resource_errors)

    return errors
