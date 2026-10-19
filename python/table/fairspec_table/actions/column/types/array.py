from __future__ import annotations

from fairspec_metadata import ArrayColumn, CellError

from fairspec_table.models import ColumnMapping
from fairspec_table.models.table import Table

from .._shared.helpers import inspect_json_column


def inspect_array_column(
    column: ArrayColumn, table: Table, mapping: ColumnMapping | None = None
) -> list[CellError]:
    return inspect_json_column(column, table, mapping=mapping)
