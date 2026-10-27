from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_metadata import (
    Dataset,
    Descriptor,
    FairspecException,
    Report,
    assert_dataset,
    convert_descriptor_from_frictionless,
    create_report,
    get_is_frictionless_descriptor,
    infer_resource_name,
    map_frictionless_errors,
    validate_dataset_descriptor,
)

from fairspec_dataset import DatasetPlugin
from fairspec_dataset.helpers.concurrency import run_concurrent_tasks

from fairspec_library.actions.dataset.load import load_dataset_model
from fairspec_library.actions.dataset.foreign_key import validate_dataset_foreign_keys
from fairspec_library.actions.resource.validate import validate_resource
from fairspec_library.models.table import ValidateTableOptions
from fairspec_library.system import system

if TYPE_CHECKING:
    from fairspec_metadata import FairspecError, Resource


def validate_dataset(
    source: Dataset | Descriptor | str, **options: Unpack[ValidateTableOptions]
) -> Report:
    def validate_loaded_dataset(dataset: Dataset) -> Report:
        return _validate_loaded_dataset(dataset, **options)

    if isinstance(source, str):
        for plugin in system.plugins:
            if isinstance(plugin, DatasetPlugin):
                report = plugin.validate_dataset(
                    source, validate_dataset=validate_loaded_dataset
                )
                if report is not None:
                    return report

        try:
            source = load_dataset_model(source)
        except FairspecException as exception:
            if exception.report:
                return exception.report
            return create_report()
    elif isinstance(source, dict):
        if get_is_frictionless_descriptor(source, kind="dataset"):
            return _validate_frictionless_dataset(source, **options)
        try:
            source = assert_dataset(source)
        except FairspecException as exception:
            if exception.report:
                return exception.report
            return create_report()

    return _validate_loaded_dataset(source, **options)


def _validate_frictionless_dataset(
    source: Descriptor, **options: Unpack[ValidateTableOptions]
) -> Report:
    report = validate_dataset_descriptor(source)
    if report.dataset is None:
        return create_report(report.errors)

    dataset_report = _validate_loaded_dataset(report.dataset, **options)
    mapping = convert_descriptor_from_frictionless(source, kind="dataset").mapping
    return create_report(map_frictionless_errors(dataset_report.errors, mapping))


def _validate_loaded_dataset(
    dataset: Dataset, **options: Unpack[ValidateTableOptions]
) -> Report:
    dataset = dataset.model_copy(deep=True)
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
