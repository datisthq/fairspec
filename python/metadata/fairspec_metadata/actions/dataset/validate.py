from __future__ import annotations

from pydantic import TypeAdapter

from fairspec_metadata.actions.data_schema.validate import validate_data_schema
from fairspec_metadata.actions.descriptor.load import load_descriptor
from fairspec_metadata.actions.descriptor.inspect import inspect_model
from fairspec_metadata.actions.descriptor.validate import validate_descriptor
from fairspec_metadata.actions.file_dialect.inspect import inspect_file_dialect
from fairspec_metadata.actions.file_dialect.validate import validate_file_dialect
from fairspec_metadata.actions.frictionless.convert import prepare_frictionless_descriptor
from fairspec_metadata.actions.frictionless.mapping import map_frictionless_errors
from fairspec_metadata.actions.profile.load import load_profile
from fairspec_metadata.actions.report.create import create_report
from fairspec_metadata.actions.resource.data import get_data_paths
from fairspec_metadata.actions.table_schema.inspect import (
    inspect_table_schema,
    inspect_table_schema_nulls,
)
from fairspec_metadata.actions.table_schema.foreign_key import inspect_foreign_key_types
from fairspec_metadata.actions.table_schema.validate import validate_table_schema
from fairspec_metadata.models.dataset import Dataset
from fairspec_metadata.models.resource import Resource
from fairspec_metadata.models.table_schema import TableSchema
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
    loaded = load_descriptor(source) if isinstance(source, str) else source

    prepared = prepare_frictionless_descriptor(loaded, kind="dataset")
    if prepared.descriptor is None:
        return DatasetValidationResult(valid=False, errors=list(prepared.errors))
    descriptor = prepared.descriptor

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
        resources = normalized.resources or []
        table_schemas: list[TableSchema | None] = [None] * len(resources)
        for index, resource in enumerate(resources):
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
            elif resource.fileDialect is not None:
                report.errors.extend(
                    inspect_file_dialect(
                        resource.fileDialect,
                        root_json_pointer=f"{root_json_pointer}/fileDialect",
                    )
                )

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
                table_schemas[index] = table_schema_result.table_schema
            elif resource.tableSchema:
                table_schemas[index] = resource.tableSchema
                report.errors.extend(
                    inspect_table_schema(
                        resource.tableSchema,
                        root_json_pointer=f"{root_json_pointer}/tableSchema",
                    )
                )
                report.errors.extend(
                    _inspect_foreign_key_targets(
                        resource.tableSchema,
                        resources,
                        root_json_pointer=f"{root_json_pointer}/tableSchema",
                    )
                )
                report.errors.extend(
                    inspect_table_schema_nulls(
                        _get_table_schema_descriptor(descriptor, index),
                        root_json_pointer=f"{root_json_pointer}/tableSchema",
                    )
                )

        report.errors.extend(_inspect_foreign_keys_types(resources, table_schemas))

        if report.errors:
            normalized = None
            report.valid = False

    errors = report.errors
    if prepared.mapping is not None:
        errors = map_frictionless_errors(errors, prepared.mapping)

    return DatasetValidationResult(
        valid=report.valid,
        errors=errors,
        dataset=normalized,
    )


def _inspect_foreign_key_targets(
    table_schema: TableSchema,
    resources: list[Resource],
    *,
    root_json_pointer: str,
) -> list[MetadataError]:
    errors: list[MetadataError] = []
    for index, foreign_key in enumerate(table_schema.foreignKeys or []):
        name = foreign_key.reference.resource
        target = next((item for item in resources if item.name == name), None)
        if name is not None and target is not None and target.dataSchema:
            errors.append(
                MetadataError(
                    type="metadata",
                    message=f'foreign key references resource "{name}", not a table',
                    jsonPointer=f"{root_json_pointer}/foreignKeys/{index}/reference/resource",
                )
            )
    return errors


def _inspect_foreign_keys_types(
    resources: list[Resource], table_schemas: list[TableSchema | None]
) -> list[MetadataError]:
    errors: list[MetadataError] = []

    for index, table_schema in enumerate(table_schemas):
        if table_schema is None:
            continue
        for fk_index, foreign_key in enumerate(table_schema.foreignKeys or []):
            name = foreign_key.reference.resource
            if name is None:
                continue
            target = next(
                (i for i, item in enumerate(resources) if item.name == name), None
            )
            if target is None or resources[target].dataSchema:
                continue
            referenced_schema = table_schemas[target]
            if referenced_schema is None:
                continue
            errors.extend(
                inspect_foreign_key_types(
                    foreign_key,
                    table_schema=table_schema,
                    referenced_schema=referenced_schema,
                    json_pointer=f"/resources/{index}/tableSchema/foreignKeys/{fk_index}",
                )
            )

    return errors


def _get_table_schema_descriptor(descriptor: Descriptor, index: int) -> Descriptor:
    resources = descriptor.get("resources")
    resource = resources[index] if isinstance(resources, list) else None
    table_schema = resource.get("tableSchema") if isinstance(resource, dict) else None
    return table_schema if isinstance(table_schema, dict) else {}
