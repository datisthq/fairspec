from __future__ import annotations

import polars as pl
import pytest
from fairspec_dataset import write_temp_file
from fairspec_metadata import (
    CsvFileDialect,
    FairspecException,
    IntegerColumnProperty,
    Resource,
    StringColumnProperty,
    TableSchema,
    TsvFileDialect,
)

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

    def test_should_fail_with_a_file_header_report_on_a_part_whose_header_differs(self):
        path1 = write_temp_file("id,name\n1,english\n")
        path2 = write_temp_file("id,nom\n2,german\n")

        with pytest.raises(FairspecException) as error:
            load_csv_table(Resource(data=[path1, path2], fileDialect=CsvFileDialect()))
        assert error.value.report is not None
        assert [item.model_dump() for item in error.value.report.errors] == [
            {
                "type": "file/header",
                "resourceName": None,
                "partNumber": 2,
                "columnNames": ["id", "nom"],
            }
        ]

    def test_should_compare_the_own_header_of_every_part_under_column_names(self):
        path1 = write_temp_file("a,b\n1,english\n")
        path2 = write_temp_file("a,c\n2,german\n")
        dialect = CsvFileDialect(columnNames=["id", "name"])

        with pytest.raises(FairspecException) as error:
            load_csv_table(Resource(data=[path1, path2], fileDialect=dialect))
        assert error.value.report is not None
        assert error.value.report.errors[0].partNumber == 2  # type: ignore[union-attr]
        assert error.value.report.errors[0].columnNames == ["a", "c"]  # type: ignore[union-attr]

    def test_should_fail_with_a_file_header_report_on_an_empty_part_under_a_header(
        self,
    ):
        path1 = write_temp_file("id,name\n1,english\n")
        path2 = write_temp_file("")

        with pytest.raises(FairspecException) as error:
            load_csv_table(Resource(data=[path1, path2], fileDialect=CsvFileDialect()))
        assert error.value.report is not None
        assert error.value.report.errors[0].partNumber == 2  # type: ignore[union-attr]
        assert error.value.report.errors[0].columnNames == []  # type: ignore[union-attr]

    def test_should_skip_an_empty_part_without_a_header(self):
        path1 = write_temp_file("1,english\n")
        path2 = write_temp_file("")

        table = load_csv_table(
            Resource(data=[path1, path2], fileDialect=CsvFileDialect(headerRows=False))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"column1": 1, "column2": "english"}]

    def test_should_join_the_header_rows_of_every_part(self):
        path1 = write_temp_file("a,b\nid,name\n1,english\n")
        path2 = write_temp_file("a,b\nid,name\n2,german\n")

        table = load_csv_table(
            Resource(data=[path1, path2], fileDialect=CsvFileDialect(headerRows=[1, 2]))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"a id": 1, "b name": "english"},
            {"a id": 2, "b name": "german"},
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

    def test_should_split_cells_on_a_multi_byte_delimiter(self):
        path = write_temp_file('id¦name\n1¦"alice¦smith"\n2¦bob\n')

        table = load_csv_table(
            Resource(data=path, fileDialect=CsvFileDialect(delimiter="¦"))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "alice¦smith"},
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

    def test_should_take_default_header_rows_when_dialect_declares_only_format(self):
        path = write_temp_file("1,100\n2,200\n3,300")

        table = load_csv_table(Resource(data=path, fileDialect=CsvFileDialect()))
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"1": 2, "100": 200},
            {"1": 3, "100": 300},
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

    def test_should_handle_crlf_line_terminator(self):
        path = write_temp_file("id,name\r\n1,alice\r\n2,bob\r\n")

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(lineTerminator="\r\n"),
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

    def test_should_number_comment_rows_before_comment_prefix_removes_rows(self):
        path = write_temp_file("id,name\n#x,y\na,b\nskip,me\nc,d\n")

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(commentRows=[4], commentPrefix="#"),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": "a", "name": "b"},
            {"id": "c", "name": "d"},
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

    def test_should_fail_on_a_row_longer_than_the_header(self):
        path = write_temp_file("id,name\n1,a,extra\n")

        with pytest.raises(Exception, match="more fields"):
            load_csv_table(Resource(data=path, fileDialect=CsvFileDialect())).collect()

    def test_should_pad_a_short_row_with_nulls(self):
        path = write_temp_file("id,name\n1\n")

        table = load_csv_table(Resource(data=path, fileDialect=CsvFileDialect()))
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"id": 1, "name": None}]

    def test_should_fail_on_a_repeated_header_name(self):
        path = write_temp_file("id,id\n1,2\n")

        with pytest.raises(Exception, match="repeated"):
            load_csv_table(Resource(data=path, fileDialect=CsvFileDialect()))

    def test_should_skip_blank_lines_without_numbering_them(self):
        path = write_temp_file("id\n\n1\n\n2\n3\n")

        table = load_csv_table(
            Resource(data=path, fileDialect=CsvFileDialect(commentRows=[3]))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"id": 1}, {"id": 3}]

    def test_should_keep_a_quoted_null_sequence_as_text(self):
        path = write_temp_file('id,name\n1,\\N\n2,"\\N"\n')

        table = load_csv_table(
            Resource(
                data=path,
                fileDialect=CsvFileDialect(nullSequence="\\N"),
                tableSchema=TableSchema(
                    properties={
                        "id": IntegerColumnProperty(),
                        "name": StringColumnProperty(),
                    }
                ),
            )
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"id": 1, "name": None}, {"id": 2, "name": "\\N"}]
