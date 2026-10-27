from __future__ import annotations

from pydantic import TypeAdapter

from fairspec_metadata.actions.data_schema.validate import validate_data_schema
from fairspec_metadata.actions.descriptor.load import load_descriptor
from fairspec_metadata.actions.descriptor.inspect import inspect_model
from fairspec_metadata.actions.descriptor.validate import validate_descriptor
from fairspec_metadata.actions.file_dialect.validate import validate_file_dialect
from fairspec_metadata.actions.profile.load import load_profile
from fairspec_metadata.actions.report.create import create_report
from fairspec_metadata.actions.resource.data import get_data_paths
from fairspec_metadata.actions.table_schema.inspect import (
    inspect_table_schema,
    inspect_table_schema_nulls,
)
from fairspec_metadata.actions.table_schema.validate import validate_table_schema
from fairspec_metadata.models.dataset import Dataset
from fairspec_metadata.models.descriptor import Descriptor
from fairspec_metadata.models.error.metadata import MetadataError
from fairspec_metadata.models.profile import ProfileType
from fairspec_metadata.models.report import Report

from .normalize import normalize_dataset


class DatasetValidationResult(Report):
    dataset: Dataset | None = None


def validate_dataset_descriptor(
    source: Descriptor | str,
    *,
    basepath: str | None = None,
) -> DatasetValidationResult:
    descriptor = load_descriptor(source) if isinstance(source, str) else source

    schema = descriptor.get("$schema")
    schema_url = (
        schema
        if isinstance(schema, str)
        else "https://fairspec.org/profiles/latest/dataset.json"
    )

    profile = load_profile(schema_url, profile_type=ProfileType.dataset)

    report = validate_descriptor(descriptor, profile=profile)

    normalized: Dataset | None = None
    if report.valid:
        dataset, model_errors = inspect_model(descriptor, model=TypeAdapter(Dataset))
        report = create_report([*report.errors, *model_errors])
        if dataset:
            normalized = normalize_dataset(dataset, basepath=basepath)

    if normalized:
        names: set[str] = set()
        for index, resource in enumerate(normalized.resources or []):
            root_json_pointer = f"/resources/{index}"

            if resource.name is not None:
                if resource.name in names:
                    report.errors.append(
                        MetadataError(
                            type="metadata",
                            message=f'resource name "{resource.name}" is not unique',
                            jsonPointer=f"{root_json_pointer}/name",
                        )
                    )
                names.add(resource.name)

            if resource.dataSchema and len(get_data_paths(resource)) > 1:
                report.errors.append(
                    MetadataError(
                        type="metadata",
                        message="a data schema needs single-part data",
                        jsonPointer=f"{root_json_pointer}/dataSchema",
                    )
                )

            if isinstance(resource.fileDialect, str):
                file_dialect_result = validate_file_dialect(
                    resource.fileDialect,
                    root_json_pointer=root_json_pointer,
                )
                report.errors.extend(file_dialect_result.errors)

            if isinstance(resource.dataSchema, str):
                data_schema_result = validate_data_schema(
                    resource.dataSchema,
                    root_json_pointer=root_json_pointer,
                )
                report.errors.extend(data_schema_result.errors)

            if isinstance(resource.tableSchema, str):
                table_schema_result = validate_table_schema(
                    resource.tableSchema,
                    root_json_pointer=root_json_pointer,
                )
                report.errors.extend(table_schema_result.errors)
            elif resource.tableSchema:
                report.errors.extend(
                    inspect_table_schema(
                        resource.tableSchema,
                        root_json_pointer=f"{root_json_pointer}/tableSchema",
                    )
                )
                report.errors.extend(
                    inspect_table_schema_nulls(
                        _get_table_schema_descriptor(descriptor, index),
                        root_json_pointer=f"{root_json_pointer}/tableSchema",
                    )
                )

        if report.errors:
            normalized = None
            report.valid = False

    return DatasetValidationResult(
        valid=report.valid,
        errors=report.errors,
        dataset=normalized,
    )


def _get_table_schema_descriptor(descriptor: Descriptor, index: int) -> Descriptor:
    resources = descriptor.get("resources")
    resource = resources[index] if isinstance(resources, list) else None
    table_schema = resource.get("tableSchema") if isinstance(resource, dict) else None
    return table_schema if isinstance(table_schema, dict) else {}
