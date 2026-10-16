from __future__ import annotations

from io import BytesIO
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_table.models.data import DataRow


def encode_xlsx_buffer(
    rows: list[DataRow],
    *,
    sheet_name: str = "Sheet1",
    book_type: str = "xlsx",
) -> bytes:
    if book_type == "ods":
        return _encode_ods(rows, sheet_name=sheet_name)
    return _encode_xlsx(rows, sheet_name=sheet_name)


def _encode_xlsx(rows: list[DataRow], *, sheet_name: str) -> bytes:
    import openpyxl

    wb = openpyxl.Workbook()
    ws = wb.active
    ws.title = sheet_name
    for row in rows:
        ws.append(row)
    buf = BytesIO()
    wb.save(buf)
    return buf.getvalue()


def _encode_ods(rows: list[DataRow], *, sheet_name: str) -> bytes:
    from odf.opendocument import OpenDocumentSpreadsheet
    from odf.table import Table, TableRow

    doc = OpenDocumentSpreadsheet()
    table = Table(name=sheet_name)

    for row_data in rows:
        tr = TableRow()
        for value in row_data:
            tc = _create_ods_cell(value)
            tr.addElement(tc)
        table.addElement(tr)

    doc.spreadsheet.addElement(table)
    buf = BytesIO()
    doc.save(buf)
    return buf.getvalue()


def _create_ods_cell(value: object) -> object:
    from odf.table import TableCell
    from odf.text import P

    if value is None:
        return TableCell()
    if isinstance(value, bool):
        tc = TableCell(valuetype="boolean", booleanvalue=str(value).lower())
        tc.addElement(P(text=str(value).upper()))
        return tc
    if isinstance(value, int):
        tc = TableCell(valuetype="float", value=str(value))
        tc.addElement(P(text=str(value)))
        return tc
    if isinstance(value, float):
        tc = TableCell(valuetype="float", value=str(value))
        tc.addElement(P(text=str(value)))
        return tc
    tc = TableCell(valuetype="string")
    tc.addElement(P(text=str(value)))
    return tc
