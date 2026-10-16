from __future__ import annotations

from fairspec_metadata.models.data_schema import DataSchema
from fairspec_metadata.models.descriptor import Descriptor
from fairspec_metadata.models.exception import FairspecException

from .validate import validate_data_schema


def assert_data_schema(source: Descriptor) -> DataSchema:
    result = validate_data_schema(source)

    if not result.data_schema:
        raise FairspecException("Invalid Data Schema", report=result)

    return result.data_schema
