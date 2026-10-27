from __future__ import annotations

import struct

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
        data = bytes.fromhex(source)
        if _skip_wkb_geometry(data, 0) is None:
            return None
        return shapely.from_wkb(data)

    return inspect_text_column(column, table, mapping=mapping, parse=parse_wkb)


def _skip_wkb_geometry(data: bytes, position: int) -> int | None:
    """The offset after the geometry at `position`, or None on a byte order other
    than 0 or 1, which GEOS reads as the machine's"""
    if position >= len(data) or data[position] not in (0, 1):
        return None
    order = "<" if data[position] == 1 else ">"
    position += 1

    def read_u32() -> int | None:
        nonlocal position
        if position + 4 > len(data):
            return None
        (value,) = struct.unpack_from(f"{order}I", data, position)
        position += 4
        return value

    type_int = read_u32()
    if type_int is None:
        return None
    geometry_type = (type_int & 0xFFFF) % 1000
    iso_type_range = (type_int & 0xFFFF) // 1000
    has_z = bool(type_int & 0x8000_0000) or iso_type_range in (1, 3)
    has_m = bool(type_int & 0x4000_0000) or iso_type_range in (2, 3)
    point_size = (2 + has_z + has_m) * 8
    if type_int & 0x2000_0000 and read_u32() is None:
        return None

    if geometry_type == 1:
        position += point_size
    elif geometry_type in (2, 3):
        lines = 1 if geometry_type == 2 else read_u32()
        if lines is None:
            return None
        for _ in range(lines):
            count = read_u32()
            if count is None:
                return None
            position += count * point_size
    elif 4 <= geometry_type <= 7:
        count = read_u32()
        if count is None:
            return None
        for _ in range(count):
            next_position = _skip_wkb_geometry(data, position)
            if next_position is None:
                return None
            position = next_position
    else:
        return None
    return position if position <= len(data) else None
