from __future__ import annotations

import zipfile
from io import BytesIO
from typing import TypedDict
from xml.sax.saxutils import escape

import xlsxwriter

from .table import format_sheet_cell

SheetValue = str | int | float | bool | None


class SheetData(TypedDict):
    name: str
    rows: list[list[SheetValue]]


def write_xlsx_buffer(sheets: list[SheetData]) -> bytes:
    """Write sheets as an xlsx workbook."""
    buffer = BytesIO()
    book = xlsxwriter.Workbook(buffer, {"in_memory": True})

    for sheet_data in sheets:
        sheet = book.add_worksheet(sheet_data["name"])
        for row, values in enumerate(sheet_data["rows"]):
            for col, value in enumerate(values):
                if isinstance(value, bool):
                    sheet.write_boolean(row, col, value)
                elif isinstance(value, int | float):
                    sheet.write_number(row, col, value)
                elif isinstance(value, str):
                    sheet.write_string(row, col, value)

    book.close()
    return buffer.getvalue()


def write_ods_buffer(sheets: list[SheetData]) -> bytes:
    """Write sheets as an ods document."""
    tables = "".join(
        f'<table:table table:name="{_escape_xml(sheet["name"])}">'
        + "".join(
            "<table:table-row>"
            + "".join(_create_ods_cell(value) for value in row)
            + "</table:table-row>"
            for row in sheet["rows"]
        )
        + "</table:table>"
        for sheet in sheets
    )

    content = (
        '<?xml version="1.0" encoding="UTF-8"?>'
        f'<office:document-content xmlns:office="{OFFICE_NS}" xmlns:table="{TABLE_NS}" '
        f'xmlns:text="{TEXT_NS}" office:version="1.2"><office:body><office:spreadsheet>'
        f"{tables}</office:spreadsheet></office:body></office:document-content>"
    )

    buffer = BytesIO()
    with zipfile.ZipFile(buffer, "w", zipfile.ZIP_DEFLATED) as archive:
        archive.writestr("mimetype", ODS_MIMETYPE, compress_type=zipfile.ZIP_STORED)
        archive.writestr("META-INF/manifest.xml", ODS_MANIFEST)
        archive.writestr("content.xml", content)

    return buffer.getvalue()


OFFICE_NS = "urn:oasis:names:tc:opendocument:xmlns:office:1.0"
TABLE_NS = "urn:oasis:names:tc:opendocument:xmlns:table:1.0"
TEXT_NS = "urn:oasis:names:tc:opendocument:xmlns:text:1.0"
ODS_MIMETYPE = "application/vnd.oasis.opendocument.spreadsheet"
ODS_MANIFEST = (
    '<?xml version="1.0" encoding="UTF-8"?>'
    '<manifest:manifest xmlns:manifest="urn:oasis:names:tc:opendocument:xmlns:manifest:1.0" '
    'manifest:version="1.2">'
    f'<manifest:file-entry manifest:full-path="/" manifest:media-type="{ODS_MIMETYPE}"/>'
    '<manifest:file-entry manifest:full-path="content.xml" manifest:media-type="text/xml"/>'
    "</manifest:manifest>"
)


# calamine rejects whitespace between cells, so the document is written without any
def _create_ods_cell(value: SheetValue) -> str:
    if value is None:
        return "<table:table-cell/>"

    if isinstance(value, bool):
        text = "true" if value else "false"
        return (
            f'<table:table-cell office:value-type="boolean" office:boolean-value="{text}">'
            f"<text:p>{text.upper()}</text:p></table:table-cell>"
        )

    if isinstance(value, int | float):
        text = format_sheet_cell(float(value))
        return (
            f'<table:table-cell office:value-type="float" office:value="{text}">'
            f"<text:p>{text}</text:p></table:table-cell>"
        )

    return (
        '<table:table-cell office:value-type="string">'
        f"<text:p>{_escape_xml(value)}</text:p></table:table-cell>"
    )


def _escape_xml(text: str) -> str:
    return escape(text, {'"': "&quot;"})
