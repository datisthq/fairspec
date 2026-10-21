from __future__ import annotations

import io
import zipfile

import pytest
from fairspec_dataset import write_temp_file

from ..guard import assert_sheet_size

TOO_LARGE = "Sheet is too large to load"


class TestAssertSheetSize:
    def test_should_accept_a_small_sheet(self):
        path = _write_xlsx(
            '<row r="1"><c r="A1"><v>1</v></c><c r="B1"><v>2</v></c></row>'
        )
        assert_sheet_size(path)

    def test_should_reject_a_sheet_naming_a_far_cell(self):
        path = _write_xlsx(
            '<row r="1"><c r="A1"><v>1</v></c></row>'
            '<row r="1048576"><c r="XFD1048576"><v>1</v></c></row>'
        )
        with pytest.raises(Exception, match=TOO_LARGE):
            assert_sheet_size(path)

    def test_should_accept_a_far_cell_without_a_value(self):
        path = _write_xlsx(
            '<row r="1"><c r="A1"><v>1</v></c></row>'
            '<row r="1048576"><c r="XFD1048576" s="1"/></row>'
        )
        assert_sheet_size(path)

    def test_should_read_a_reference_after_a_quoted_gt(self):
        path = _write_xlsx(
            '<row r="1"><c r="A1"><v>1</v></c></row>'
            '<row><c t="n" x=">" r="XFD1048576"><v>1</v></c></row>'
        )
        with pytest.raises(Exception, match=TOO_LARGE):
            assert_sheet_size(path)

    def test_should_place_cells_without_a_reference_one_after_another(self):
        wide = "<c><v>1</v></c>" * 10_001
        path = _write_xlsx(
            f'<row r="1">{wide}</row><row r="5001"><c r="A5001"><v>1</v></c></row>'
        )
        with pytest.raises(Exception, match=TOO_LARGE):
            assert_sheet_size(path)

    def test_should_reject_ods_cells_repeated_over_a_large_range(self):
        path = _write_ods(
            '<table:table-row table:number-rows-repeated="1048576">'
            '<table:table-cell table:number-columns-repeated="16384" '
            'office:value-type="float" office:value="1"><text:p>1</text:p>'
            "</table:table-cell></table:table-row>"
        )
        with pytest.raises(Exception, match=TOO_LARGE):
            assert_sheet_size(path)

    def test_should_accept_empty_ods_cells_repeated_to_the_sheet_edge(self):
        path = _write_ods(
            '<table:table-row><table:table-cell office:value-type="float" '
            'office:value="1"><text:p>1</text:p></table:table-cell>'
            '<table:table-cell table:number-columns-repeated="16383"/></table:table-row>'
            '<table:table-row table:number-rows-repeated="1048575">'
            '<table:table-cell table:number-columns-repeated="16384"/></table:table-row>'
        )
        assert_sheet_size(path)

    def test_should_reject_an_xlsb_workbook(self):
        path = write_temp_file(_zip({"xl/workbook.bin": ""}))
        with pytest.raises(Exception, match="xlsb is not supported"):
            assert_sheet_size(path)

    def test_should_skip_a_file_that_is_not_a_zip_archive(self):
        path = write_temp_file("id,name\n1,a\n")
        assert_sheet_size(path)


def _write_xlsx(sheet_data: str) -> str:
    sheet = (
        '<?xml version="1.0" encoding="UTF-8"?><worksheet '
        'xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">'
        f"<sheetData>{sheet_data}</sheetData></worksheet>"
    )
    return write_temp_file(_zip({"xl/worksheets/sheet1.xml": sheet}))


def _write_ods(rows: str) -> str:
    content = (
        '<?xml version="1.0" encoding="UTF-8"?><office:document-content '
        'xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" '
        'xmlns:table="urn:oasis:names:tc:opendocument:xmlns:table:1.0" '
        'xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0">'
        "<office:body><office:spreadsheet>"
        f'<table:table table:name="Sheet1">{rows}</table:table>'
        "</office:spreadsheet></office:body></office:document-content>"
    )
    return write_temp_file(_zip({"content.xml": content}))


def _zip(files: dict[str, str]) -> bytes:
    buffer = io.BytesIO()
    with zipfile.ZipFile(buffer, "w", zipfile.ZIP_DEFLATED) as archive:
        for name, text in files.items():
            archive.writestr(name, text)
    return buffer.getvalue()
