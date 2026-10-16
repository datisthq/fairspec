from __future__ import annotations

import polars as pl

from fairspec_table.models.column import PolarsColumn
from fairspec_table.models.schema import PolarsSchema


def get_polars_schema(type_mapping: dict[str, pl.DataType]) -> PolarsSchema:
    columns = [
        PolarsColumn(name=name, type=type(dtype)) for name, dtype in type_mapping.items()
    ]
    return PolarsSchema(columns=columns)
