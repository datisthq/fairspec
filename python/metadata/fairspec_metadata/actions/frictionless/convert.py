from __future__ import annotations

import re
from dataclasses import dataclass, field
from typing import Literal

from fairspec_metadata.actions.dataset.from_frictionless import (
    convert_dataset_from_frictionless,
)
from fairspec_metadata.actions.file_dialect.from_frictionless import (
    convert_file_dialect_from_frictionless,
)
from fairspec_metadata.actions.table_schema.from_frictionless import (
    convert_table_schema_from_frictionless,
)
from fairspec_metadata.models.descriptor import Descriptor
from fairspec_metadata.models.error.metadata import MetadataError
from fairspec_metadata.models.frictionless.dialect import FrictionlessDialect
from fairspec_metadata.models.frictionless.package import FrictionlessPackage
from fairspec_metadata.models.frictionless.schema import FrictionlessSchema

from .inspect import inspect_frictionless_model
from .mapping import FrictionlessMapping, create_frictionless_recorder

FrictionlessKind = Literal["dataset", "tableSchema", "fileDialect"]

FRICTIONLESS_RESOURCE_KEYS = ("path", "schema", "dialect")
FRICTIONLESS_DIALECT_KEYS = (
    "header",
    "headerRows",
    "headerJoin",
    "commentRows",
    "commentChar",
    "delimiter",
    "lineTerminator",
    "quoteChar",
    "doubleQuote",
    "escapeChar",
    "nullSequence",
    "skipInitialSpace",
    "property",
    "itemType",
    "itemKeys",
    "sheetNumber",
    "sheetName",
    "table",
)


@dataclass
class FrictionlessConversion:
    descriptor: Descriptor | None
    mapping: FrictionlessMapping
    errors: list[MetadataError] = field(default_factory=list)


@dataclass
class FrictionlessPreparation:
    descriptor: Descriptor | None
    mapping: FrictionlessMapping | None = None
    errors: list[MetadataError] = field(default_factory=list)


def get_is_frictionless_descriptor(
    descriptor: Descriptor, *, kind: FrictionlessKind
) -> bool:
    """Whether a descriptor is the Frictionless counterpart of a Dataset, Table Schema or
    File Dialect, judged by its content; a Fairspec `$schema` always wins
    """
    schema = descriptor.get("$schema")
    schema = schema if isinstance(schema, str) else ""
    if schema.startswith("https://fairspec.org/"):
        return False

    match kind:
        case "tableSchema":
            return (
                isinstance(descriptor.get("fields"), list)
                and "properties" not in descriptor
            )
        case "fileDialect":
            return "format" not in descriptor and any(
                key in descriptor for key in FRICTIONLESS_DIALECT_KEYS
            )
        case "dataset":
            if isinstance(descriptor.get("profile"), str):
                return True
            if re.search(r"datapackage\.org|frictionlessdata\.io", schema):
                return True
            resources = descriptor.get("resources")
            return isinstance(resources, list) and any(
                isinstance(resource, dict)
                and any(key in resource for key in FRICTIONLESS_RESOURCE_KEYS)
                for resource in resources
            )


def convert_descriptor_from_frictionless(
    descriptor: Descriptor,
    *,
    kind: FrictionlessKind,
    root_json_pointer: str | None = None,
) -> FrictionlessConversion:
    """Convert a Frictionless descriptor to its Fairspec counterpart, with the mapping that
    points errors about the result back at the original

    A descriptor the Frictionless model cannot read comes back as metadata errors instead
    """
    mapping = FrictionlessMapping()
    root = "" if root_json_pointer in (None, "/") else root_json_pointer

    match kind:
        case "dataset":
            package, errors = inspect_frictionless_model(
                descriptor, model=FrictionlessPackage, root_json_pointer=root
            )
            if package is None:
                return FrictionlessConversion(None, mapping, errors)
            converted = convert_dataset_from_frictionless(package, mapping=mapping)
            return FrictionlessConversion(converted, mapping)
        case "tableSchema":
            schema, errors = inspect_frictionless_model(
                descriptor, model=FrictionlessSchema, root_json_pointer=root
            )
            if schema is None:
                return FrictionlessConversion(None, mapping, errors)
            record = create_frictionless_recorder(
                mapping, fairspec_prefix=root, frictionless_prefix=root
            )
            converted = convert_table_schema_from_frictionless(schema, record=record)
            return FrictionlessConversion(converted, mapping)
        case "fileDialect":
            dialect, errors = inspect_frictionless_model(
                descriptor, model=FrictionlessDialect, root_json_pointer=root
            )
            if dialect is None:
                return FrictionlessConversion(None, mapping, errors)
            record = create_frictionless_recorder(
                mapping, fairspec_prefix=root, frictionless_prefix=root
            )
            converted = convert_file_dialect_from_frictionless(dialect, record=record)
            return FrictionlessConversion(converted, mapping)


def prepare_frictionless_descriptor(
    descriptor: Descriptor,
    *,
    kind: FrictionlessKind,
    root_json_pointer: str | None = None,
) -> FrictionlessPreparation:
    """Convert a descriptor when it is Frictionless, else pass it through unchanged"""
    if not get_is_frictionless_descriptor(descriptor, kind=kind):
        return FrictionlessPreparation(descriptor)
    conversion = convert_descriptor_from_frictionless(
        descriptor, kind=kind, root_json_pointer=root_json_pointer
    )
    return FrictionlessPreparation(
        conversion.descriptor, conversion.mapping, conversion.errors
    )
