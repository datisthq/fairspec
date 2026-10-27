from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import (
    FairspecException,
    assert_dataset,
    convert_descriptor_from_frictionless,
    create_report,
    load_descriptor,
    map_frictionless_errors,
    resolve_basepath,
)

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


def load_dataset_from_frictionless(path: str) -> Descriptor:
    """Load a descriptor as a Frictionless Data Package (v1 or v2), whatever its content

    `load_dataset` recognises one by content without this
    """
    basepath = resolve_basepath(path)
    descriptor = load_descriptor(path)

    conversion = convert_descriptor_from_frictionless(descriptor, kind="dataset")
    if conversion.descriptor is None:
        raise FairspecException(
            "Invalid Frictionless Data Package",
            report=create_report(list(conversion.errors)),
        )

    try:
        dataset = assert_dataset(conversion.descriptor, basepath=basepath)
    except FairspecException as exception:
        if exception.report is None:
            raise
        errors = map_frictionless_errors(exception.report.errors, conversion.mapping)
        raise FairspecException(str(exception), report=create_report(errors)) from None

    return dataset.model_dump(by_alias=True, exclude_none=True)
