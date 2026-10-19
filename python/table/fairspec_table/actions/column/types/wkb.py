from __future__ import annotations

import shapely

from fairspec_metadata import CellError, WkbColumn

from fairspec_table.models import ColumnMapping
from fairspec_table.models.table import Table

from .._shared.helpers import inspect_text_column


def inspect_wkb_column(
    column: WkbColumn, table: Table, mapping: ColumnMapping | None = None
) -> list[CellError]:
    return inspect_text_column(column, table, mapping=mapping, parse=_parse_wkb)


def _parse_wkb(source: str) -> object:
    return shapely.from_wkb(bytes.fromhex(source))
