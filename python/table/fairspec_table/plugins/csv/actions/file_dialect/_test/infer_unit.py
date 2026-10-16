from __future__ import annotations

from fairspec_dataset import write_temp_file
from fairspec_metadata import CsvFileDialect, Resource, TsvFileDialect

from ..infer import infer_csv_file_dialect


class TestInferCsvFileDialectBasic:
    def test_should_infer_simple_csv_file(self):
        path = write_temp_file("id,name\n1,english\n2,中文")

        result = infer_csv_file_dialect(Resource(data=path))

        assert result == CsvFileDialect(
            delimiter=",", headerRows=[1], lineTerminator="\n"
        )

    def test_should_infer_quote_char(self):
        path = write_temp_file('id,name\n1,"John Doe"\n2,"Jane Smith"')

        result = infer_csv_file_dialect(Resource(data=path))

        assert result == CsvFileDialect(
            delimiter=",", quoteChar='"', headerRows=[1], lineTerminator="\n"
        )

    def test_should_infer_quote_char_with_single_quotes(self):
        path = write_temp_file("id,name\n1,'John Doe'\n2,'Jane Smith'")

        result = infer_csv_file_dialect(Resource(data=path))

        assert result == CsvFileDialect(
            delimiter=",", quoteChar="'", headerRows=[1], lineTerminator="\n"
        )

    def test_should_infer_header_false_when_no_header(self):
        path = write_temp_file("1,english\n2,中文\n3,español")

        result = infer_csv_file_dialect(Resource(data=path))

        assert result == CsvFileDialect(
            delimiter=",", headerRows=False, lineTerminator="\n"
        )

    def test_should_detect_header_when_present(self):
        path = write_temp_file("id,name\n1,english\n2,中文")

        result = infer_csv_file_dialect(Resource(data=path))

        assert result == CsvFileDialect(
            delimiter=",", headerRows=[1], lineTerminator="\n"
        )

    def test_should_infer_complex_csv_with_quotes_and_header(self):
        path = write_temp_file(
            'name,description\n"Product A","A great product with, commas"\n"Product B","Another product"'
        )

        result = infer_csv_file_dialect(Resource(data=path))

        assert result == CsvFileDialect(
            delimiter=",", quoteChar='"', headerRows=[1], lineTerminator="\n"
        )


class TestInferCsvFileDialectDelimiters:
    def test_should_infer_comma_delimiter(self):
        path = write_temp_file("id,name,age\n1,alice,25\n2,bob,30")

        result = infer_csv_file_dialect(Resource(data=path, fileDialect=CsvFileDialect()))

        assert result == CsvFileDialect(
            delimiter=",", headerRows=[1], lineTerminator="\n"
        )

    def test_should_infer_pipe_delimiter(self):
        path = write_temp_file("id|name|age\n1|alice|25\n2|bob|30")

        result = infer_csv_file_dialect(Resource(data=path, fileDialect=CsvFileDialect()))

        assert result == CsvFileDialect(
            delimiter="|", headerRows=[1], lineTerminator="\n"
        )

    def test_should_infer_semicolon_delimiter(self):
        path = write_temp_file("id;name;age\n1;alice;25\n2;bob;30")

        result = infer_csv_file_dialect(Resource(data=path, fileDialect=CsvFileDialect()))

        assert result == CsvFileDialect(
            delimiter=";", headerRows=[1], lineTerminator="\n"
        )

    def test_should_infer_tab_delimiter_as_tsv(self):
        path = write_temp_file("id\tname\tage\n1\talice\t25\n2\tbob\t30")

        result = infer_csv_file_dialect(Resource(data=path, fileDialect=CsvFileDialect()))

        assert result == TsvFileDialect(headerRows=[1], lineTerminator="\n")


class TestInferCsvFileDialectQuotes:
    def test_should_handle_quoted_fields(self):
        path = write_temp_file(
            'id,name,description\n1,"alice","Description with, comma"\n2,"bob","Normal text"'
        )

        result = infer_csv_file_dialect(Resource(data=path, fileDialect=CsvFileDialect()))

        assert result == CsvFileDialect(
            delimiter=",", quoteChar='"', headerRows=[1], lineTerminator="\n"
        )

    def test_should_handle_single_quote_character(self):
        path = write_temp_file(
            "id,name,description\n1,'alice','Description text'\n2,'bob','Normal text'"
        )

        result = infer_csv_file_dialect(Resource(data=path, fileDialect=CsvFileDialect()))

        assert result == CsvFileDialect(
            delimiter=",", quoteChar="'", headerRows=[1], lineTerminator="\n"
        )


class TestInferCsvFileDialectEdgeCases:
    def test_should_return_none_for_resources_without_path(self):
        resource = Resource(
            data=[
                {"id": 1, "name": "alice"},
                {"id": 2, "name": "bob"},
            ],
        )

        result = infer_csv_file_dialect(resource)

        assert result is None

    def test_should_handle_custom_line_terminator(self):
        path = write_temp_file("id,name\r\n1,alice\r\n2,bob\r\n")

        result = infer_csv_file_dialect(Resource(data=path, fileDialect=CsvFileDialect()))

        assert result == CsvFileDialect(
            delimiter=",", headerRows=[1], lineTerminator="\r\n"
        )

    def test_should_handle_header_row_only(self):
        path = write_temp_file("id,name,age")

        result = infer_csv_file_dialect(Resource(data=path, fileDialect=CsvFileDialect()))

        assert result == CsvFileDialect(
            delimiter=",", headerRows=False, lineTerminator="\n"
        )

    def test_should_handle_empty_file(self):
        path = write_temp_file("")

        result = infer_csv_file_dialect(Resource(data=path, fileDialect=CsvFileDialect()))

        assert result == CsvFileDialect(delimiter=",", lineTerminator="\n")


class TestInferCsvFileDialectHeaderDetection:
    def test_should_detect_header_with_mixed_types(self):
        path = write_temp_file("id,name,age\n1,Alice,25\n2,Bob,30\n3,Charlie,35")

        result = infer_csv_file_dialect(Resource(data=path))

        assert result == CsvFileDialect(
            delimiter=",", headerRows=[1], lineTerminator="\n"
        )

    def test_should_detect_header_after_preamble_rows(self):
        path = write_temp_file(
            "# Comment line 1\n# Comment line 2\nid,name,age\n1,Alice,25\n2,Bob,30"
        )

        result = infer_csv_file_dialect(Resource(data=path))

        assert result == CsvFileDialect(
            delimiter=",", headerRows=[3], lineTerminator="\n"
        )

    def test_should_not_detect_header_when_first_row_is_numeric(self):
        path = write_temp_file("1,2,3\n4,5,6\n7,8,9")

        result = infer_csv_file_dialect(Resource(data=path))

        assert result == CsvFileDialect(
            delimiter=",", headerRows=False, lineTerminator="\n"
        )

    def test_should_detect_header_with_underscores_and_mixed_case(self):
        path = write_temp_file(
            "user_id,User_Name,EmailAddress\n1,alice,alice@example.com\n2,bob,bob@example.com"
        )

        result = infer_csv_file_dialect(Resource(data=path))

        assert result == CsvFileDialect(
            delimiter=",", headerRows=[1], lineTerminator="\n"
        )

    def test_should_not_detect_header_when_first_row_has_data_like_values(self):
        path = write_temp_file(
            "blsrpxedd,37257,695.80,false,1927-11-07T01:03:54Z\n"
            "zmvpq03o4,68694,337.73,false,1927-04-02T12:37:52Z\n"
            "iw1fm3k9n,52019,988.74,false,2009-02-22T05:50:15Z"
        )

        result = infer_csv_file_dialect(Resource(data=path))

        assert result == CsvFileDialect(
            delimiter=",", headerRows=False, lineTerminator="\n"
        )
