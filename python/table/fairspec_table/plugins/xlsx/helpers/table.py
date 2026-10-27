from __future__ import annotations

import datetime
import math
from decimal import Decimal
from typing import TYPE_CHECKING, Literal

import polars as pl

from fairspec_table.helpers.file_dialect import (
    assert_unique_column_names,
    get_header_rows,
    join_header_labels,
)

if TYPE_CHECKING:
    from fairspec_metadata import OdsFileDialect, XlsxFileDialect

    from .sheet import SheetCell

SheetType = Literal["number", "boolean", "date", "string"]


def create_table_from_sheet_rows(
    rows: list[list[SheetCell]],
    file_dialect: XlsxFileDialect | OdsFileDialect,
) -> pl.LazyFrame:
    """Build a table from sheet rows.

    Data starts after the last header row; a column keeps its cells' type when
    they share one and becomes text when they mix.
    """
    header_rows = get_header_rows(file_dialect)
    data_start = max(header_rows, default=0)
    content = rows[data_start:]

    labels = file_dialect.columnNames
    if labels is None:
        labels = _get_sheet_labels(rows, content, header_rows, file_dialect)
        assert_unique_column_names(labels)

    first_row = header_rows[0] - 1 if header_rows else 0
    width = max((_get_row_width(row) for row in rows[first_row:]), default=0)
    if file_dialect.columnNames is not None and width > len(labels):
        raise Exception(
            f"The sheet has {width} columns but columnNames has {len(labels)}"
        )

    columns = [
        [row[index] if index < len(row) else None for row in content]
        for index in range(len(labels))
    ]
    types = [_get_column_type(column) for column in columns]

    comment_rows = file_dialect.commentRows or []
    comment_prefix = file_dialect.commentPrefix
    kept: list[int] = []
    for index, row in enumerate(content):
        if data_start + index + 1 in comment_rows:
            continue
        first = row[0] if row else None
        if comment_prefix and isinstance(first, str) and first.startswith(comment_prefix):
            continue
        kept.append(index)

    series = [
        _create_sheet_series(label, [column[row] for row in kept], kind)
        for label, column, kind in zip(labels, columns, types, strict=True)
    ]

    # Polars names an unnamed series of a new frame by its position; an empty name stays
    frame = pl.DataFrame(series)
    frame.columns = list(labels)
    return frame.lazy()


def get_sheet_header(
    rows: list[list[SheetCell]],
    file_dialect: XlsxFileDialect | OdsFileDialect,
) -> list[str]:
    """The column names a sheet's own header rows give, whatever columnNames says"""
    header_rows = get_header_rows(file_dialect)
    content = rows[max(header_rows, default=0) :]
    return _get_sheet_labels(rows, content, header_rows, file_dialect)


def format_sheet_cell(cell: SheetCell) -> str | None:
    """Format a cell as the text a mixed-type column holds."""
    if cell is None:
        return None
    if isinstance(cell, bool):
        return "true" if cell else "false"
    if isinstance(cell, int | float):
        return _format_number(float(cell))
    if isinstance(cell, datetime.datetime):
        return _format_date(cell)
    return cell


def _get_row_width(row: list[SheetCell]) -> int:
    return max(
        (index + 1 for index, cell in enumerate(row) if cell is not None), default=0
    )


def _get_sheet_labels(
    rows: list[list[SheetCell]],
    content: list[list[SheetCell]],
    header_rows: list[int],
    file_dialect: XlsxFileDialect | OdsFileDialect,
) -> list[str]:
    if not header_rows:
        width = max((len(row) for row in content), default=0)
        return [f"column{index + 1}" for index in range(width)]

    for number in header_rows:
        if number - 1 >= len(rows):
            raise Exception(f"Header row {number} is past the end of the sheet")
    header: list[list[object]] = [
        [format_sheet_cell(cell) or "" for cell in rows[number - 1]]
        for number in header_rows
    ]

    if len(header) <= 1:
        return [str(label) for label in header[0]] if header else []
    header_join = file_dialect.headerJoin
    return join_header_labels(header, " " if header_join is None else header_join)


def _get_column_type(cells: list[SheetCell]) -> SheetType:
    kind: SheetType | None = None
    for cell in cells:
        if cell is None:
            continue
        cell_kind = _get_cell_type(cell)
        if kind and kind != cell_kind:
            return "string"
        kind = cell_kind

    return kind or "string"


def _get_cell_type(cell: SheetCell) -> SheetType:
    if isinstance(cell, bool):
        return "boolean"
    if isinstance(cell, int | float):
        return "number"
    if isinstance(cell, datetime.datetime):
        return "date"
    return "string"


def _create_sheet_series(
    name: str, values: list[SheetCell], kind: SheetType
) -> pl.Series:
    if kind == "number":
        return pl.Series(name, values, dtype=pl.Float64)
    if kind == "boolean":
        return pl.Series(name, values, dtype=pl.Boolean)
    if kind == "date":
        return pl.Series(name, values, dtype=pl.Datetime("ms"))
    return pl.Series(
        name, [format_sheet_cell(value) for value in values], dtype=pl.String
    )


def _format_date(value: datetime.datetime) -> str:
    text = value.strftime("%Y-%m-%dT%H:%M:%S")
    milliseconds = value.microsecond // 1000
    return f"{text}.{milliseconds:03d}" if milliseconds else text


# Matches JavaScript's String(number), which TypeScript's twin uses
def _format_number(value: float) -> str:
    if math.isnan(value):
        return "NaN"
    if math.isinf(value):
        return "Infinity" if value > 0 else "-Infinity"
    if value == 0:
        return "0"

    sign = "-" if value < 0 else ""
    parts = Decimal(repr(abs(value))).normalize().as_tuple()
    digits = "".join(str(digit) for digit in parts.digits)
    k = len(digits)
    n = k + int(parts.exponent)

    if k <= n <= 21:
        return sign + digits + "0" * (n - k)
    if 0 < n <= 21:
        return sign + digits[:n] + "." + digits[n:]
    if -6 < n <= 0:
        return sign + "0." + "0" * -n + digits

    exponent = n - 1
    mantissa = digits[0] + ("." + digits[1:] if k > 1 else "")
    return f"{sign}{mantissa}e{'+' if exponent >= 0 else '-'}{abs(exponent)}"
