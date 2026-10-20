from __future__ import annotations

from fairspec_metadata import CellError, WkbColumn

from fairspec_table.models import ColumnMapping
from fairspec_table.models.table import Table

from .._shared.helpers import inspect_text_column


def inspect_wkb_column(
    column: WkbColumn, table: Table, mapping: ColumnMapping | None = None
) -> list[CellError]:
    # imported here: shapely pulls in numpy, about 50ms on every CLI start
    import shapely

    def parse_wkb(source: str) -> object:
        return shapely.from_wkb(bytes.fromhex(source))

    return inspect_text_column(column, table, mapping=mapping, parse=parse_wkb)
