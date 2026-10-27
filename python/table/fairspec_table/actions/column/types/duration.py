from __future__ import annotations

import isodate

from fairspec_metadata import CellError, DurationColumn

from fairspec_table.models import ColumnMapping
from fairspec_table.models.table import Table

from .._shared.helpers import inspect_text_column


def inspect_duration_column(
    column: DurationColumn, table: Table, mapping: ColumnMapping | None = None
) -> list[CellError]:
    return inspect_text_column(column, table, mapping=mapping, parse=_parse_duration)


def _parse_duration(text: str) -> object:
    # isodate reads `PT` and `P1DT`, though ISO 8601 needs a component after the `T`
    if text.removesuffix("\n").endswith("T"):
        raise ValueError(f"A time designator without components: {text}")

    return isodate.parse_duration(text)
