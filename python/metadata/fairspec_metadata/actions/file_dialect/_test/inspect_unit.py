from __future__ import annotations

from fairspec_metadata.models.file_dialect.common import RowType
from fairspec_metadata.models.file_dialect.csv import CsvFileDialect
from fairspec_metadata.models.file_dialect.jsonl import JsonlFileDialect

from ..inspect import inspect_file_dialect


class TestInspectFileDialect:
    def test_should_report_a_row_listed_in_header_rows_and_comment_rows(self):
        errors = inspect_file_dialect(
            CsvFileDialect(headerRows=[1, 2], commentRows=[2]),
            root_json_pointer="/fileDialect",
        )

        assert [error.jsonPointer for error in errors] == ["/fileDialect/commentRows"]

    def test_should_take_the_default_header_row(self):
        assert len(inspect_file_dialect(CsvFileDialect(commentRows=[1]))) == 1

    def test_should_accept_comment_rows_without_a_header(self):
        dialect = CsvFileDialect(headerRows=False, commentRows=[1])
        assert inspect_file_dialect(dialect) == []

    def test_should_skip_object_rows_which_comment_rows_does_not_apply_to(self):
        dialect = JsonlFileDialect(rowType=RowType.object, commentRows=[1])
        assert inspect_file_dialect(dialect) == []
