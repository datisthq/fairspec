from __future__ import annotations

import datetime
from collections.abc import Sequence
from typing import TYPE_CHECKING

from python_calamine import CalamineWorkbook

if TYPE_CHECKING:
    from fairspec_metadata import OdsFileDialect, XlsxFileDialect

SheetCell = str | float | bool | datetime.datetime | None


def read_sheet_rows(
    path: str,
    file_dialect: XlsxFileDialect | OdsFileDialect,
    *,
    max_rows: int | None = None,
) -> list[list[SheetCell]] | None:
    """Read the dialect's sheet as rows of cells, without trailing empty cells.

    Returns None when the workbook has no such sheet.
    """
    index = file_dialect.sheetNumber - 1 if file_dialect.sheetNumber else 0

    workbook = CalamineWorkbook.from_path(path)
    try:
        names = workbook.sheet_names
        name = file_dialect.sheetName
        if name is None:
            name = names[index] if index < len(names) else None
        if name is None or name not in names:
            return None
        rows = workbook.get_sheet_by_name(name).to_python(nrows=max_rows)
    finally:
        workbook.close()

    return [_normalize_row(row) for row in rows]


def _normalize_row(row: Sequence[object]) -> list[SheetCell]:
    cells = [_normalize_cell(cell) for cell in row]
    while cells and cells[-1] is None:
        cells.pop()
    return cells


# TypeScript's twin maps calamine-node cells to the same values: a time-only value
# lands on 1899-12-31 and a duration is a number of days
def _normalize_cell(cell: object) -> SheetCell:
    if cell == "" or cell is None:
        return None
    if isinstance(cell, bool | str):
        return cell
    if isinstance(cell, int | float):
        return float(cell)
    if isinstance(cell, datetime.datetime):
        return _round_milliseconds(cell)
    if isinstance(cell, datetime.date):
        return datetime.datetime.combine(cell, datetime.time())
    if isinstance(cell, datetime.time):
        epoch = datetime.date(1899, 12, 31)
        return _round_milliseconds(datetime.datetime.combine(epoch, cell))
    if isinstance(cell, datetime.timedelta):
        return cell.total_seconds() / 86400
    return None


def _round_milliseconds(value: datetime.datetime) -> datetime.datetime:
    milliseconds = round(value.microsecond / 1000)
    return value.replace(microsecond=0) + datetime.timedelta(milliseconds=milliseconds)
