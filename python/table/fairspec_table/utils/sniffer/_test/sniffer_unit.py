from __future__ import annotations

from datetime import datetime, timezone

from ..metadata import Quote
from ..sample import SampleSize, SampleSizeType
from ..sniffer import Sniffer


class TestSniffBytes:
    def test_detect_comma_delimited_csv(self):
        csv = "id,name,age\n1,Alice,25\n2,Bob,30"
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.dialect.delimiter == 44
        assert metadata.dialect.header.has_header_row is True
        assert metadata.num_fields == 3
        assert metadata.fields == ["id", "name", "age"]

    def test_detect_tab_delimited_tsv(self):
        tsv = "id\tname\tage\n1\tAlice\t25\n2\tBob\t30"
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(tsv.encode())

        assert metadata.dialect.delimiter == 9
        assert metadata.dialect.header.has_header_row is True
        assert metadata.num_fields == 3

    def test_detect_semicolon_delimited(self):
        csv = "id;name;age\n1;Alice;25\n2;Bob;30"
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.dialect.delimiter == 59
        assert metadata.num_fields == 3

    def test_detect_pipe_delimited(self):
        csv = "id|name|age\n1|Alice|25\n2|Bob|30"
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.dialect.delimiter == 124
        assert metadata.num_fields == 3

    def test_detect_quoted_fields(self):
        csv = (
            'id,name,description\n1,"Alice","She said, ""Hello"""\n2,"Bob","Normal text"'
        )
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.dialect.delimiter == 44
        assert metadata.dialect.quote.char is not None
        assert metadata.dialect.quote.char == 34

    def test_detect_crlf(self):
        csv = "id,name\r\n1,Alice\r\n2,Bob"
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.dialect.delimiter == 44
        assert metadata.num_fields == 2

    def test_detect_cr(self):
        csv = "id,name\r1,Alice\r2,Bob"
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.dialect.delimiter == 44
        assert metadata.num_fields == 2

    def test_detect_no_header(self):
        csv = "1,Alice,25\n2,Bob,30\n3,Charlie,35"
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.dialect.header.has_header_row is False
        assert metadata.fields == ["field_1", "field_2", "field_3"]

    def test_skip_comment_preamble(self):
        csv = "# This is a comment\n# Another comment\nid,name\n1,Alice\n2,Bob"
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.dialect.header.num_preamble_rows == 2
        assert metadata.fields == ["id", "name"]

    def test_detect_structural_preamble(self):
        csv = "Report Title\nReport Date: 2024-01-01\nid,name\n1,Alice\n2,Bob"
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.dialect.header.num_preamble_rows >= 1

    def test_handle_utf8_bom(self):
        bom = bytes([0xEF, 0xBB, 0xBF])
        csv = "id,name\n1,Alice".encode()
        data = bom + csv

        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(data)

        assert metadata.dialect.delimiter == 44
        assert metadata.fields == ["id", "name"]

    def test_flexible_mode(self):
        csv = "id,name\n1,Alice\n2,Bob,Extra\n3,Charlie"
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.dialect.flexible is True

    def test_force_delimiter(self):
        csv = "id;name;age\n1;Alice;25\n2;Bob;30"
        sniffer = Sniffer().with_delimiter(59)
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.dialect.delimiter == 59

    def test_force_quote(self):
        csv = "id,'name','age'\n1,'Alice','25'"
        sniffer = Sniffer().with_quote(Quote(char=39))
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.dialect.quote.char is not None
        assert metadata.dialect.quote.char == 39

    def test_avg_record_len(self):
        csv = "id,name\n1,Alice\n2,Bob"
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.avg_record_len > 0

    def test_empty_file(self):
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(b"")

        assert metadata.num_fields == 0

    def test_single_line(self):
        csv = "id,name,age"
        sniffer = Sniffer()
        metadata = sniffer.sniff_bytes(csv.encode())

        assert metadata.num_fields == 3


class TestSniffRows:
    def test_detect_comma_with_header(self):
        rows: list[list[object]] = [
            ["id", "name", "age"],
            [1, "Alice", 25],
            [2, "Bob", 30],
        ]
        sniffer = Sniffer()
        metadata = sniffer.sniff_rows(rows)

        assert metadata.dialect.delimiter == 44
        assert metadata.dialect.header.has_header_row is True
        assert metadata.fields == ["id", "name", "age"]
        assert metadata.num_fields == 3

    def test_detect_no_header(self):
        rows: list[list[object]] = [
            [1, "Alice", 25],
            [2, "Bob", 30],
            [3, "Charlie", 35],
        ]
        sniffer = Sniffer()
        metadata = sniffer.sniff_rows(rows)

        assert metadata.dialect.delimiter == 44
        assert metadata.dialect.header.has_header_row is False
        assert metadata.fields == ["field_1", "field_2", "field_3"]

    def test_string_values_with_commas(self):
        rows: list[list[object]] = [
            ["name", "city"],
            ["Smith, John", "New York"],
            ["Doe, Jane", "Los Angeles"],
        ]
        sniffer = Sniffer()
        metadata = sniffer.sniff_rows(rows)

        assert metadata.dialect.delimiter == 44
        assert metadata.fields == ["name", "city"]

    def test_string_values_with_quotes(self):
        rows: list[list[object]] = [
            ["text", "author"],
            ['He said "Hello"', "Alice"],
            ['She said "Hi"', "Bob"],
        ]
        sniffer = Sniffer()
        metadata = sniffer.sniff_rows(rows)

        assert metadata.dialect.quote.char is not None
        assert metadata.dialect.quote.char == 34

    def test_null_to_empty_string(self):
        rows: list[list[object]] = [
            ["id", "name", "optional"],
            [1, "Alice", None],
            [2, "Bob", None],
        ]
        sniffer = Sniffer()
        metadata = sniffer.sniff_rows(rows)

        assert metadata.fields == ["id", "name", "optional"]

    def test_numbers_and_booleans(self):
        rows: list[list[object]] = [
            ["count", "price", "active"],
            [42, 19.99, True],
            [100, 5.5, False],
        ]
        sniffer = Sniffer()
        metadata = sniffer.sniff_rows(rows)

        assert metadata.num_fields == 3

    def test_date_objects(self):
        rows: list[list[object]] = [
            ["event", "timestamp"],
            ["Login", datetime(2024, 1, 1, 10, 0, 0, tzinfo=timezone.utc)],
            ["Logout", datetime(2024, 1, 1, 11, 0, 0, tzinfo=timezone.utc)],
        ]
        sniffer = Sniffer()
        metadata = sniffer.sniff_rows(rows)

        assert metadata.fields == ["event", "timestamp"]

    def test_objects_and_arrays(self):
        rows: list[list[object]] = [
            ["id", "metadata", "tags"],
            [1, {"key": "value"}, ["a", "b"]],
            [2, {"key": "other"}, ["c"]],
        ]
        sniffer = Sniffer()
        metadata = sniffer.sniff_rows(rows)

        assert metadata.num_fields == 3

    def test_variable_lengths_flexible(self):
        rows: list[list[object]] = [
            ["id", "name"],
            [1, "Alice"],
            [2, "Bob", "extra"],
            [3],
        ]
        sniffer = Sniffer()
        metadata = sniffer.sniff_rows(rows)

        assert metadata.dialect.flexible is True

    def test_empty_array(self):
        rows: list[list[object]] = []
        sniffer = Sniffer()
        metadata = sniffer.sniff_rows(rows)

        assert metadata.num_fields == 0
        assert metadata.fields == []

    def test_single_row(self):
        rows: list[list[object]] = [[1, "Alice"]]
        sniffer = Sniffer()
        metadata = sniffer.sniff_rows(rows)

        assert metadata.num_fields == 2

    def test_configured_sample_size(self):
        rows: list[list[object]] = [[i, f"data{i}"] for i in range(1000)]
        sniffer = Sniffer().with_sample_size(
            SampleSize(type=SampleSizeType.RECORDS, count=10)
        )
        metadata = sniffer.sniff_rows(rows)

        assert metadata.num_fields == 2

    def test_preamble_rows_with_hash(self):
        rows: list[list[object]] = [
            ["# Exported from Excel on 2024-01-01"],
            ["# Data source: Sales Report"],
            ["id", "product", "quantity"],
            [1, "Widget", 100],
            [2, "Gadget", 150],
        ]
        sniffer = Sniffer()
        metadata = sniffer.sniff_rows(rows)

        assert metadata.dialect.header.num_preamble_rows == 2
        assert metadata.dialect.header.has_header_row is True
        assert metadata.fields == ["id", "product", "quantity"]
        assert metadata.num_fields == 3
