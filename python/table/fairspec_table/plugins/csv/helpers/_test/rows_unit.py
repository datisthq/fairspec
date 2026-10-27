from __future__ import annotations

from fairspec_dataset import write_temp_file
from fairspec_metadata import CsvFileDialect, TsvFileDialect

from ..rows import clean_csv_rows


def read_text(path: str) -> str:
    with open(path, encoding="utf-8", newline="") as file:
        return file.read()


class TestCleanCsvRows:
    def test_should_keep_files_without_a_blank_line_or_a_null_sequence(self):
        path = write_temp_file("a,b\r\n1,2\r\n")
        assert clean_csv_rows([path], CsvFileDialect()) is None

    def test_should_drop_blank_lines_but_not_one_inside_a_quoted_cell(self):
        path = write_temp_file('\na,b\n1,"x\n\ny"\n\r\n3,4\n\n')
        cleaned = clean_csv_rows([path], CsvFileDialect())

        assert cleaned is not None
        assert read_text(cleaned.paths[0]) == 'a,b\n1,"x\n\ny"\n3,4\n'

    def test_should_keep_a_row_of_delimiters(self):
        path = write_temp_file("a,b\n,\n\n")
        cleaned = clean_csv_rows([path], CsvFileDialect())

        assert cleaned is not None
        assert read_text(cleaned.paths[0]) == "a,b\n,\n"

    def test_should_empty_only_unquoted_data_cells_equal_to_the_null_sequence(self):
        path = write_temp_file('NA,b\nNA,"NA"\n"a""NA",NA\r\n')
        cleaned = clean_csv_rows([path], CsvFileDialect(nullSequence="NA"))

        assert cleaned is not None
        assert cleaned.file_dialect.nullSequence is None
        assert read_text(cleaned.paths[0]) == 'NA,b\n,"NA"\n"a""NA",\r\n'

    def test_should_keep_the_quotes_of_tsv_cells(self):
        path = write_temp_file('a\tb\n"NA\tNA"\n')
        cleaned = clean_csv_rows([path], TsvFileDialect(nullSequence="NA"))

        assert cleaned is not None
        assert read_text(cleaned.paths[0]) == 'a\tb\n"NA\tNA"\n'

    def test_should_not_read_quotes_in_a_comment_row(self):
        path = write_temp_file("a\n# it's\n\nx\n")
        cleaned = clean_csv_rows([path], CsvFileDialect(quoteChar="'", commentPrefix="#"))

        assert cleaned is not None
        assert read_text(cleaned.paths[0]) == "a\n# it's\nx\n"
