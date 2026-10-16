from __future__ import annotations

import polars as pl
import pytest
from fairspec_dataset import write_temp_file
from fairspec_metadata import CsvFileDialect, Resource, TsvFileDialect

from ..load import load_csv_table


class TestLoadCsvTable:
    def test_should_load_local_file(self):
        path = write_temp_file("id,name\n1,english\n2,中文")

        table = load_csv_table(Resource(data=path, fileDialect=CsvFileDialect()))
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_load_local_file_multipart(self):
        path1 = write_temp_file("id,name\n1,english")
        path2 = write_temp_file("id,name\n2,中文\n3,german")

        table = load_csv_table(
            Resource(data=[path1, path2], fileDialect=CsvFileDialect())
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
            {"id": 3, "name": "german"},
        ]

    @pytest.mark.vcr
    def test_should_load_remote_file_with_preview_bytes(self):
        table = load_csv_table(
            Resource(
                data="https://raw.githubusercontent.com/datisthq/fairspec-typescript/refs/heads/main/table/plugins/csv/actions/table/-test/fixtures/table.csv",
            ),
            previewBytes=18,
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
        ]

    def test_should_handle_custom_delimiter(self):
        path = write_temp_file("id|name\n1|alice\n2|bob")

        table = load_csv_table(
            Resource(data=path, fileDialect=CsvFileDialect(delimiter="|"))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "alice"},
            {"id": 2, "name": "bob"},
        ]

    def test_should_handle_files_without_header(self):
        path = write_temp_file("1,alice\n2,bob")

        table = load_csv_table(
            Resource(data=path, fileDialect=CsvFileDialect(headerRows=False))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"column1": 1, "column2": "alice"},
            {"column1": 2, "column2": "bob"},
        ]

    def test_should_handle_files_without_header_using_column_names(self):
        path = write_temp_file("1,alice\n2,bob")

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(headerRows=False, columnNames=["id", "name"]),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "alice"},
            {"id": 2, "name": "bob"},
        ]

    def test_should_infer_header_rows_when_partial_dialect(self):
        path = write_temp_file("1,100\n2,200\n3,300")

        table = load_csv_table(Resource(data=path, fileDialect=CsvFileDialect()))
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"column1": 1, "column2": 100},
            {"column1": 2, "column2": 200},
            {"column1": 3, "column2": 300},
        ]

    def test_should_handle_custom_line_terminator(self):
        path = write_temp_file("id,name|1,alice|2,bob")

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(lineTerminator="|"),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "alice"},
            {"id": 2, "name": "bob"},
        ]

    def test_should_handle_custom_quote_character(self):
        path = write_temp_file("id,name\n1,'alice smith'\n2,'bob jones'")

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(quoteChar="'"),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "alice smith"},
            {"id": 2, "name": "bob jones"},
        ]

    def test_should_handle_comment_character(self):
        path = write_temp_file(
            "# This is a comment\nid,name\n1,alice\n# Another comment\n2,bob"
        )

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(commentPrefix="#"),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "alice"},
            {"id": 2, "name": "bob"},
        ]

    def test_should_support_header_rows(self):
        path = write_temp_file("#comment\nid,name\n1,alice\n2,bob")

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(headerRows=[2]),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "alice"},
            {"id": 2, "name": "bob"},
        ]

    def test_should_support_header_join(self):
        path = write_temp_file("#comment\nid,name\nint,str\n1,alice\n2,bob")

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(headerRows=[2, 3], headerJoin="_"),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id_int": 1, "name_str": "alice"},
            {"id_int": 2, "name_str": "bob"},
        ]

    def test_should_support_comment_rows(self):
        path = write_temp_file("id,name\n1,alice\ncomment\n2,bob")

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(commentRows=[3]),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "alice"},
            {"id": 2, "name": "bob"},
        ]

    def test_should_support_header_rows_and_comment_rows(self):
        path = write_temp_file("#comment\nid,name\n1,alice\n#comment\n2,bob")

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(headerRows=[2], commentRows=[4]),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "alice"},
            {"id": 2, "name": "bob"},
        ]

    def test_should_support_header_join_and_comment_rows(self):
        path = write_temp_file("#comment\nid,name\nint,str\n1,alice\n#comment\n2,bob")

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(
                    headerRows=[2, 3], headerJoin="_", commentRows=[5]
                ),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id_int": 1, "name_str": "alice"},
            {"id_int": 2, "name_str": "bob"},
        ]

    def test_should_handle_null_sequence(self):
        path = write_temp_file("id,name,age\n1,alice,25\n2,N/A,30\n3,bob,N/A")

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(nullSequence="N/A"),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "alice", "age": 25},
            {"id": 2, "name": None, "age": 30},
            {"id": 3, "name": "bob", "age": None},
        ]

    def test_should_handle_multiple_format_options_together(self):
        path = write_temp_file(
            "#comment\nid|'full name'|age\n1|'alice smith'|25\n2|'bob jones'|30"
        )

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(
                    delimiter="|",
                    quoteChar="'",
                    commentPrefix="#",
                    headerRows=[1],
                ),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "full name": "alice smith", "age": 25},
            {"id": 2, "full name": "bob jones", "age": 30},
        ]


class TestLoadCsvTableTsv:
    def test_should_load_local_file(self):
        path = write_temp_file("id\tname\n1\tenglish\n2\t中文")

        table = load_csv_table(Resource(data=path, fileDialect=TsvFileDialect()))
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]
