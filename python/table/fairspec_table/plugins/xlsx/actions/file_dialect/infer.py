from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_dataset import load_file
from fairspec_metadata import (
    OdsFileDialect,
    XlsxFileDialect,
    get_data_first_path,
    get_supported_file_dialect,
)

from fairspec_table.plugins.xlsx.actions.buffer.decode import decode_xlsx_buffer
from fairspec_table.utils.sniffer.sniffer import Sniffer

if TYPE_CHECKING:
    from fairspec_metadata import Resource

    from fairspec_table.models.table import LoadTableOptions


def infer_xlsx_file_dialect(
    resource: Resource,
    options: LoadTableOptions | None = None,
) -> XlsxFileDialect | OdsFileDialect | None:
    data_path = get_data_first_path(resource)
    if not data_path:
        return None

    file_dialect = get_supported_file_dialect(resource, ["xlsx", "ods"])
    if not file_dialect:
        return None

    format: str = getattr(file_dialect, "format", "xlsx")

    try:
        buffer = load_file(data_path)
    except Exception:
        return _make_dialect(format)

    try:
        sheet_name: str | None = getattr(file_dialect, "sheetName", None)
        sheet_number: int | None = getattr(file_dialect, "sheetNumber", None)
        rows = decode_xlsx_buffer(
            buffer,
            format=format,
            sheet_name=sheet_name,
            sheet_number=sheet_number,
        )
    except Exception:
        return _make_dialect(format)

    if not rows:
        return _make_dialect(format)

    sniffer = Sniffer()
    try:
        detection = sniffer.sniff_rows(rows[:100])
    except Exception:
        return _make_dialect(format)

    header_rows: list[int] | bool | None = None
    if detection.dialect.header.has_header_row:
        header_rows = [detection.dialect.header.num_preamble_rows + 1]
    elif detection.num_fields > 0:
        header_rows = False

    return _make_dialect(format, header_rows)


def _make_dialect(
    format: str,
    header_rows: list[int] | bool | None = None,
) -> XlsxFileDialect | OdsFileDialect:
    kwargs: dict[str, object] = {}
    if header_rows is not None:
        kwargs["headerRows"] = header_rows
    if format == "ods":
        return OdsFileDialect(**kwargs)  # type: ignore[arg-type]
    return XlsxFileDialect(**kwargs)  # type: ignore[arg-type]
