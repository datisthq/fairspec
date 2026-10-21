from __future__ import annotations

from python_calamine import CalamineWorkbook

from fairspec_table.plugins.xlsx.helpers.write import (
    SheetData,
    SheetValue,
    write_ods_buffer,
    write_xlsx_buffer,
)

# We intentionally don't use fairspec's loader here to isolate the tests


def read_test_data(path: str) -> list[dict[str, object]]:
    workbook = CalamineWorkbook.from_path(path)
    header, *rows = workbook.get_sheet_by_index(0).to_python()
    workbook.close()

    return [
        {str(label): row[index] for index, label in enumerate(header)} for row in rows
    ]


def write_test_data(
    path: str,
    rows: list[list[SheetValue]],
    *,
    sheet_number: int = 1,
    sheet_name: str | None = None,
    format: str = "xlsx",
) -> None:
    sheets: list[SheetData] = [
        {"name": sheet_name or f"Sheet{index + 1}", "rows": rows}
        for index in range(sheet_number)
    ]

    buffer = write_ods_buffer(sheets) if format == "ods" else write_xlsx_buffer(sheets)
    with open(path, "wb") as file:
        file.write(buffer)
