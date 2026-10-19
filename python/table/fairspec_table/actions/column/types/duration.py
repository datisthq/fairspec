from __future__ import annotations

import isodate

from fairspec_metadata import CellError, DurationColumn

from fairspec_table.models import ColumnMapping
from fairspec_table.models.table import Table

from .._shared.helpers import inspect_text_column


def inspect_duration_column(
    column: DurationColumn, table: Table, mapping: ColumnMapping | None = None
) -> list[CellError]:
    return inspect_text_column(
        column, table, mapping=mapping, parse=isodate.parse_duration
    )
