from __future__ import annotations

import shapely

from fairspec_metadata import CellError, WktColumn

from fairspec_table.models import ColumnMapping
from fairspec_table.models.table import Table

from .._shared.helpers import inspect_text_column


def inspect_wkt_column(
    column: WktColumn, table: Table, mapping: ColumnMapping | None = None
) -> list[CellError]:
    return inspect_text_column(column, table, mapping=mapping, parse=shapely.from_wkt)
