from __future__ import annotations

from fairspec_metadata.models.descriptor import Descriptor
from fairspec_metadata.models.exception import FairspecException
from fairspec_metadata.models.table_schema import TableSchema

from .validate import validate_table_schema


def assert_table_schema(source: Descriptor) -> TableSchema:
    result = validate_table_schema(source)

    if not result.table_schema:
        raise FairspecException("Invalid Table Schema", report=result)

    return result.table_schema
