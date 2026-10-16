from __future__ import annotations

from ..metadata import LineTerminator, Quote
from ..potential_dialects import PotentialDialect
from ..table import Table


class TestTable:
    def test_parse_comma_delimited(self):
        csv = "a,b,c\n1,2,3\n4,5,6"
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(), line_terminator=LineTerminator.LF
        )
        table = Table.parse(csv.encode(), dialect)

        assert table.rows == [
            ["a", "b", "c"],
            ["1", "2", "3"],
            ["4", "5", "6"],
        ]
        assert table.field_counts == [3, 3, 3]

    def test_parse_quoted_fields(self):
        csv = 'a,"b,c",d\n1,"2,3",4'
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(char=34), line_terminator=LineTerminator.LF
        )
        table = Table.parse(csv.encode(), dialect)

        assert table.rows == [
            ["a", "b,c", "d"],
            ["1", "2,3", "4"],
        ]

    def test_escaped_quotes(self):
        csv = 'a,"b""c",d'
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(char=34), line_terminator=LineTerminator.LF
        )
        table = Table.parse(csv.encode(), dialect)

        assert table.rows == [["a", 'b"c', "d"]]

    def test_modal_field_count(self):
        rows = [
            ["a", "b", "c"],
            ["1", "2", "3"],
            ["4", "5"],
            ["7", "8", "9"],
        ]
        field_counts = [3, 3, 2, 3]
        table = Table(rows, field_counts)

        assert table.get_modal_field_count() == 3

    def test_cache_modal_field_count(self):
        rows = [["a", "b"], ["1", "2"]]
        field_counts = [2, 2]
        table = Table(rows, field_counts)

        first = table.get_modal_field_count()
        second = table.get_modal_field_count()

        assert first == second
        assert first == 2

    def test_detect_uniform(self):
        rows = [["a", "b"], ["1", "2"], ["3", "4"]]
        field_counts = [2, 2, 2]
        table = Table(rows, field_counts)

        assert table.is_uniform() is True

    def test_detect_non_uniform(self):
        rows = [["a", "b"], ["1", "2", "3"], ["4", "5"]]
        field_counts = [2, 3, 2]
        table = Table(rows, field_counts)

        assert table.is_uniform() is False

    def test_num_rows(self):
        rows = [["a"], ["b"], ["c"]]
        field_counts = [1, 1, 1]
        table = Table(rows, field_counts)

        assert table.num_rows() == 3

    def test_empty_table(self):
        table = Table([], [])

        assert table.num_rows() == 0
        assert table.get_modal_field_count() == 0
        assert table.is_uniform() is True

    def test_parse_crlf(self):
        csv = "a,b\r\n1,2\r\n3,4"
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(), line_terminator=LineTerminator.CRLF
        )
        table = Table.parse(csv.encode(), dialect)

        assert len(table.rows) == 3

    def test_parse_cr(self):
        csv = "a,b\r1,2\r3,4"
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(), line_terminator=LineTerminator.CR
        )
        table = Table.parse(csv.encode(), dialect)

        assert len(table.rows) == 3

    def test_modal_field_count_wide_tables(self):
        rows = [["x"] * (260 if i % 2 == 0 else 250) for i in range(300)]
        field_counts = [len(row) for row in rows]
        table = Table(rows, field_counts)

        assert table.get_modal_field_count() == 260
