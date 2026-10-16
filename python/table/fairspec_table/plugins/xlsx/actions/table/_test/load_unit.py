from __future__ import annotations

import polars as pl
from fairspec_dataset import get_temp_file_path
from fairspec_metadata import OdsFileDialect, Resource, XlsxFileDialect
from ..load import load_xlsx_table
from .test_data import write_test_data

ROW1 = ["id", "name"]
ROW2 = [1, "english"]
ROW3 = [2, "中文"]

RECORD1 = {"id": 1, "name": "english"}
RECORD2 = {"id": 2, "name": "中文"}


class TestLoadXlsxTableXlsx:
    def test_should_load_local_file(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3])

        table = load_xlsx_table(Resource(data=path, fileDialect=XlsxFileDialect()))
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2]

    def test_should_load_local_file_multipart(self):
        path1 = get_temp_file_path()
        path2 = get_temp_file_path()
        write_test_data(path1, [ROW1, ROW2, ROW3])
        write_test_data(path2, [ROW1, ROW2, ROW3])

        table = load_xlsx_table(
            Resource(data=[path1, path2], fileDialect=XlsxFileDialect())
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2, RECORD1, RECORD2]

    def test_should_support_sheet_number(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3], sheet_number=2)

        table = load_xlsx_table(
            Resource(data=path, fileDialect=XlsxFileDialect(sheetNumber=2))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2]

    def test_should_support_sheet_name(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3], sheet_name="Sheet2")

        table = load_xlsx_table(
            Resource(data=path, fileDialect=XlsxFileDialect(sheetName="Sheet2"))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2]

    def test_should_support_no_header(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW2, ROW3])

        table = load_xlsx_table(
            Resource(data=path, fileDialect=XlsxFileDialect(headerRows=False))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"column1": 1, "column2": "english"},
            {"column1": 2, "column2": "中文"},
        ]

    def test_should_support_header_rows_offset(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3])

        table = load_xlsx_table(
            Resource(data=path, fileDialect=XlsxFileDialect(headerRows=[2]))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"1": 2, "english": "中文"}]

    def test_should_support_multiline_header_rows(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3])

        table = load_xlsx_table(
            Resource(data=path, fileDialect=XlsxFileDialect(headerRows=[1, 2]))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"id 1": 2, "name english": "中文"}]

    def test_should_support_header_join(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3])

        table = load_xlsx_table(
            Resource(
                data=path,
                fileDialect=XlsxFileDialect(headerRows=[1, 2], headerJoin="-"),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"id-1": 2, "name-english": "中文"}]

    def test_should_support_comment_rows(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3])

        table = load_xlsx_table(
            Resource(data=path, fileDialect=XlsxFileDialect(commentRows=[2]))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD2]

    def test_should_support_comment_prefix(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3, ["#comment"]])

        table = load_xlsx_table(
            Resource(data=path, fileDialect=XlsxFileDialect(commentPrefix="#"))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2]

    def test_should_handle_longer_rows(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3, [3, "german", "bad"]])

        table = load_xlsx_table(
            Resource(data=path, fileDialect=XlsxFileDialect(commentPrefix="#"))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2, {"id": 3, "name": "german"}]

    def test_should_handle_shorter_rows(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3, [3]])

        table = load_xlsx_table(
            Resource(data=path, fileDialect=XlsxFileDialect(commentPrefix="#"))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2, {"id": 3, "name": None}]


class TestLoadXlsxTableOds:
    def test_should_load_local_file(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3], format="ods")

        table = load_xlsx_table(Resource(data=path, fileDialect=OdsFileDialect()))
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2]

    def test_should_load_local_file_multipart(self):
        path1 = get_temp_file_path()
        path2 = get_temp_file_path()
        write_test_data(path1, [ROW1, ROW2, ROW3], format="ods")
        write_test_data(path2, [ROW1, ROW2, ROW3], format="ods")

        table = load_xlsx_table(
            Resource(data=[path1, path2], fileDialect=OdsFileDialect())
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2, RECORD1, RECORD2]

    def test_should_support_sheet_number(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3], sheet_number=2, format="ods")

        table = load_xlsx_table(
            Resource(data=path, fileDialect=OdsFileDialect(sheetNumber=2))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2]

    def test_should_support_sheet_name(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3], sheet_name="Sheet2", format="ods")

        table = load_xlsx_table(
            Resource(data=path, fileDialect=OdsFileDialect(sheetName="Sheet2"))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2]

    def test_should_support_no_header(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW2, ROW3], format="ods")

        table = load_xlsx_table(
            Resource(data=path, fileDialect=OdsFileDialect(headerRows=False))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"column1": 1, "column2": "english"},
            {"column1": 2, "column2": "中文"},
        ]

    def test_should_support_header_rows_offset(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3], format="ods")

        table = load_xlsx_table(
            Resource(data=path, fileDialect=OdsFileDialect(headerRows=[2]))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"1": 2, "english": "中文"}]

    def test_should_support_multiline_header_rows(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3], format="ods")

        table = load_xlsx_table(
            Resource(data=path, fileDialect=OdsFileDialect(headerRows=[1, 2]))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"id 1": 2, "name english": "中文"}]

    def test_should_support_header_join(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3], format="ods")

        table = load_xlsx_table(
            Resource(
                data=path,
                fileDialect=OdsFileDialect(headerRows=[1, 2], headerJoin="-"),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"id-1": 2, "name-english": "中文"}]

    def test_should_support_comment_rows(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3], format="ods")

        table = load_xlsx_table(
            Resource(data=path, fileDialect=OdsFileDialect(commentRows=[2]))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD2]

    def test_should_support_comment_prefix(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3, ["#comment"]], format="ods")

        table = load_xlsx_table(
            Resource(data=path, fileDialect=OdsFileDialect(commentPrefix="#"))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2]

    def test_should_handle_longer_rows(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3, [3, "german", "bad"]], format="ods")

        table = load_xlsx_table(
            Resource(data=path, fileDialect=OdsFileDialect(commentPrefix="#"))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2, {"id": 3, "name": "german"}]

    def test_should_handle_shorter_rows(self):
        path = get_temp_file_path()
        write_test_data(path, [ROW1, ROW2, ROW3, [3]], format="ods")

        table = load_xlsx_table(
            Resource(data=path, fileDialect=OdsFileDialect(commentPrefix="#"))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [RECORD1, RECORD2, {"id": 3, "name": None}]
