from __future__ import annotations

from typing import Any, Union

from pydantic import Field, ValidationInfo, field_validator

from .data import Data
from .data_schema import DataSchema
from .datacite.datacite import Datacite
from .descriptor import Descriptor
from .file_dialect.file_dialect import FileDialect
from .integrity import Integrity
from .path import Path
from .table_schema import TableSchema


class Resource(Datacite):
    data: Data | None = Field(
        default=None,
        description="Data or content of the resource. It must be a path to a file, array of paths to files, inline JSON object, or inline JSON array of objects.",
    )
    name: str | None = Field(
        default=None,
        pattern=r"^[a-zA-Z0-9_]+$",
        description="An optional name for the resource consisting of alphanumeric characters and underscores. If provided, it can be used to reference resource within a dataset context.",
    )
    textual: bool | None = Field(
        default=None,
        description="Whether the resource is text-based.",
    )
    integrity: Integrity | None = Field(
        default=None,
        description="The integrity check of the file with type (md5, sha1, sha256, sha512) and hash value.",
    )
    fileDialect: Union[Path, FileDialect] | None = Field(
        default=None,
        description="A path to dialect or an object with the dialect of the file. For multiple files the format property defines the dialect for all the files.",
    )
    dataSchema: Union[Path, DataSchema] | None = Field(
        default=None,
        description="A path to a JSON Schema or an object with the JSON Schema. The JSON Schema must be compatible with the JSONSchema Draft 2020-12 specification.",
    )
    tableSchema: Union[Path, TableSchema] | None = Field(
        default=None,
        description="A path to a Table Schema or an object with the Table Schema. The Table Schema must be compatible with the Fairspec Table specification.",
    )
    unstable_customMetadata: dict[str, Any] | None = Field(
        default=None,
        description="Custom properties for extending resources",
    )

    # TypeScript converts an inline Frictionless object when it resolves one; a model
    # here could not hold it, as neither a Fairspec dialect nor schema reads its keys
    @field_validator("fileDialect", "tableSchema", mode="before")
    @classmethod
    def convert_frictionless_descriptor(
        cls, value: object, info: ValidationInfo
    ) -> object:
        from fairspec_metadata.actions.frictionless.convert import (
            convert_descriptor_from_frictionless,
            get_is_frictionless_descriptor,
        )

        if not isinstance(value, dict):
            return value
        descriptor: Descriptor = {str(key): item for key, item in value.items()}
        kind = "fileDialect" if info.field_name == "fileDialect" else "tableSchema"
        if not get_is_frictionless_descriptor(descriptor, kind=kind):
            return value
        conversion = convert_descriptor_from_frictionless(descriptor, kind=kind)
        return value if conversion.descriptor is None else conversion.descriptor
