from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import (
    assert_dataset,
    get_is_descriptor,
    load_descriptor,
    normalize_path,
    resolve_basepath,
)

from fairspec_dataset.plugins.frictionless.helpers.inspect import (
    assert_frictionless_model,
)
from fairspec_dataset.plugins.frictionless.models.dialect import FrictionlessDialect
from fairspec_dataset.plugins.frictionless.models.package import FrictionlessPackage
from fairspec_dataset.plugins.frictionless.models.schema import FrictionlessSchema
from .from_frictionless import convert_dataset_from_frictionless

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


def load_dataset_from_frictionless(path: str) -> Descriptor:
    """Load a Frictionless Data Package (v1 or v2) as a Dataset

    A `datapackage.json` holding a Fairspec Dataset loads as it is
    """
    basepath = resolve_basepath(path)
    descriptor = load_descriptor(path)

    schema = descriptor.get("$schema")
    if isinstance(schema, str) and schema.startswith("https://fairspec.org/"):
        dataset = assert_dataset(descriptor, basepath=basepath)
        return dataset.model_dump(by_alias=True, exclude_none=True)

    frictionless_package = assert_frictionless_model(
        descriptor, model=FrictionlessPackage
    )

    resources = []
    for index, resource in enumerate(frictionless_package.resources):
        json_pointer = f"/resources/{index}"
        update: Descriptor = {}
        if isinstance(resource.schema_, str):
            update["schema_"] = assert_frictionless_model(
                _load_reference(resource.schema_, basepath),
                model=FrictionlessSchema,
                json_pointer=f"{json_pointer}/schema",
            )
        if isinstance(resource.dialect, str):
            update["dialect"] = assert_frictionless_model(
                _load_reference(resource.dialect, basepath),
                model=FrictionlessDialect,
                json_pointer=f"{json_pointer}/dialect",
            )
        resources.append(resource.model_copy(update=update))

    dataset = assert_dataset(
        convert_dataset_from_frictionless(
            frictionless_package.model_copy(update={"resources": resources})
        ),
        basepath=basepath,
    )
    return dataset.model_dump(by_alias=True, exclude_none=True)


def _load_reference(path: str, basepath: str) -> Descriptor:
    descriptor = load_descriptor(normalize_path(path, basepath=basepath))
    if not get_is_descriptor(descriptor):
        raise Exception(f"Referenced descriptor is not a JSON object: {path}")
    return descriptor
