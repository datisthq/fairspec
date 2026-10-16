from __future__ import annotations

from fairspec_metadata.actions.descriptor.load import load_descriptor
from fairspec_metadata.models.data_schema import DataSchema

from .assert_ import assert_data_schema


def load_data_schema(path: str) -> DataSchema:
    descriptor = load_descriptor(path)
    return assert_data_schema(descriptor)
