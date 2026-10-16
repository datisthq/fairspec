from __future__ import annotations

from fairspec_metadata.actions.descriptor.load import load_descriptor
from fairspec_metadata.models.table_schema import TableSchema

from .assert_ import assert_table_schema


def load_table_schema(path: str) -> TableSchema:
    descriptor = load_descriptor(path)
    return assert_table_schema(descriptor)
