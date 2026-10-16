from __future__ import annotations

import json
import re
from datetime import datetime

from .metadata import Dialect, Header, LineTerminator, Metadata, Quote
from .potential_dialects import (
    PotentialDialect,
    detect_line_terminator,
    generate_potential_dialects,
)
from .sample import SampleSize, SampleSizeType
from .score import FindBestDialectPreferences, find_best_dialect, score_dialect
from .table import Table


class Sniffer:
    def __init__(self) -> None:
        self._sample_size: SampleSize = SampleSize(type=SampleSizeType.BYTES, count=8192)
        self._forced_delimiter: int | None = None
        self._forced_quote: Quote | None = None

    def with_sample_size(self, size: SampleSize) -> Sniffer:
        self._sample_size = size
        return self

    def with_delimiter(self, delimiter: int) -> Sniffer:
        self._forced_delimiter = delimiter
        return self

    def with_quote(self, quote: Quote) -> Sniffer:
        self._forced_quote = quote
        return self

    def sniff_bytes(self, data: bytes) -> Metadata:
        bytes_no_bom = self._skip_bom(data)
        current = bytes_no_bom

        without_comment_preamble, comment_preamble_lines = self._skip_preamble(current)
        current = without_comment_preamble

        sample = self._take_sample(current)

        line_terminator = detect_line_terminator(sample)

        if self._forced_delimiter is not None:
            dialects = self._generate_forced_dialects(line_terminator)
        else:
            dialects = generate_potential_dialects(line_terminator)

        scores = [score_dialect(sample, dialect) for dialect in dialects]

        best_score = find_best_dialect(
            scores,
            FindBestDialectPreferences(
                prefer_common_delimiters=True,
                prefer_double_quote=True,
            ),
        )

        dialect = Dialect(
            delimiter=best_score.dialect.delimiter,
            quote=best_score.dialect.quote,
            flexible=not best_score.is_uniform,
            is_utf8=True,
            line_terminator=best_score.dialect.line_terminator,
            header=Header(
                has_header_row=False,
                num_preamble_rows=comment_preamble_lines,
            ),
        )

        structural_preamble_rows = self._detect_structural_preamble(current, dialect)
        dialect.header.num_preamble_rows += structural_preamble_rows

        data_after_all_preamble = self._skip_lines(current, structural_preamble_rows)
        header_detection_result = self._detect_header(data_after_all_preamble, dialect)
        dialect.header.has_header_row = header_detection_result

        return self._build_metadata(bytes_no_bom, dialect)

    def sniff_rows(self, rows: list[list[object]]) -> Metadata:
        csv_string = self._rows_to_csv(rows)
        data = csv_string.encode("utf-8")
        return self.sniff_bytes(data)

    def _value_to_string(self, value: object) -> str:
        if value is None:
            return ""
        if isinstance(value, str):
            return value
        if isinstance(value, bool):
            return str(value).lower()
        if isinstance(value, (int, float)):
            return str(value)
        if isinstance(value, datetime):
            return value.isoformat()
        if isinstance(value, (dict, list)):
            return json.dumps(value)
        return str(value)

    def _escape_field(self, value: str) -> str:
        needs_quoting = bool(re.search(r'[,"\n\r]', value))

        if not needs_quoting:
            return value

        escaped = value.replace('"', '""')
        return f'"{escaped}"'

    def _rows_to_csv(self, rows: list[list[object]]) -> str:
        if len(rows) == 0:
            return ""

        lines: list[str] = []

        for row in rows:
            values = [self._escape_field(self._value_to_string(value)) for value in row]
            lines.append(",".join(values))

        return "\n".join(lines)

    def _skip_bom(self, data: bytes) -> bytes:
        if len(data) >= 3 and data[0] == 0xEF and data[1] == 0xBB and data[2] == 0xBF:
            return data[3:]
        return data

    def _skip_preamble(self, data: bytes) -> tuple[bytes, int]:
        line_count = 0
        i = 0

        while i < len(data):
            if data[i] == 35:
                while i < len(data) and data[i] != 10 and data[i] != 13:
                    i += 1

                if (
                    i < len(data)
                    and data[i] == 13
                    and i + 1 < len(data)
                    and data[i + 1] == 10
                ):
                    i += 2
                elif i < len(data):
                    i += 1

                line_count += 1
            else:
                break

        return data[i:], line_count

    def _take_sample(self, data: bytes) -> bytes:
        if self._sample_size.type == SampleSizeType.ALL:
            return data

        if self._sample_size.type == SampleSizeType.BYTES:
            return data[: self._sample_size.count]

        line_count = 0
        i = 0

        while i < len(data) and line_count < self._sample_size.count:
            if data[i] == 10:
                line_count += 1
            elif data[i] == 13:
                if i + 1 < len(data) and data[i + 1] == 10:
                    i += 1
                line_count += 1
            i += 1

        return data[:i]

    def _generate_forced_dialects(
        self,
        line_terminator: LineTerminator,
    ) -> list[PotentialDialect]:
        if self._forced_delimiter is None:
            raise ValueError("generateForcedDialects called without forcedDelimiter")

        delimiter = self._forced_delimiter

        if self._forced_quote is not None:
            quotes: list[Quote] = [self._forced_quote]
        else:
            quotes = [
                Quote(char=None),
                Quote(char=34),
                Quote(char=39),
            ]

        return [
            PotentialDialect(
                delimiter=delimiter,
                quote=quote,
                line_terminator=line_terminator,
            )
            for quote in quotes
        ]

    def _detect_structural_preamble(
        self,
        data: bytes,
        dialect: Dialect,
    ) -> int:
        table = Table.parse(
            data,
            PotentialDialect(
                delimiter=dialect.delimiter,
                quote=dialect.quote,
                line_terminator=dialect.line_terminator,
            ),
        )

        if table.num_rows() < 2:
            return 0

        modal_field_count = table.get_modal_field_count()
        preamble_rows = 0

        for i in range(len(table.field_counts)):
            count = table.field_counts[i]
            if count == modal_field_count:
                break
            preamble_rows += 1

        if preamble_rows >= table.num_rows():
            return 0

        return preamble_rows

    def _skip_lines(self, data: bytes, num_lines: int) -> bytes:
        if num_lines == 0:
            return data

        line_count = 0
        i = 0

        while i < len(data) and line_count < num_lines:
            if data[i] == 10:
                line_count += 1
                i += 1
            elif data[i] == 13:
                if i + 1 < len(data) and data[i + 1] == 10:
                    i += 2
                else:
                    i += 1
                line_count += 1
            else:
                i += 1

        return data[i:]

    def _detect_header(
        self,
        data: bytes,
        dialect: Dialect,
    ) -> bool:
        table = Table.parse(
            data,
            PotentialDialect(
                delimiter=dialect.delimiter,
                quote=dialect.quote,
                line_terminator=dialect.line_terminator,
            ),
        )

        if table.num_rows() < 2:
            return False

        first_row = table.rows[0]
        if not first_row:
            return False

        header_score = 0.0

        for field in first_row:
            trimmed = field.strip()

            if len(trimmed) == 0:
                header_score -= 0.2
                continue

            if re.fullmatch(r"(?i)^(true|false)$", trimmed):
                header_score -= 0.3
                continue

            if re.fullmatch(r"^[a-zA-Z_][a-zA-Z0-9_]*$", trimmed):
                header_score += 0.3

            if re.search(r"[A-Z]", trimmed) and not re.fullmatch(r"^\d+$", trimmed):
                header_score += 0.2

            if re.search(r"[_\s-]", trimmed):
                header_score += 0.1

            if re.fullmatch(r"^\d+(\.\d+)?$", trimmed):
                header_score -= 0.3

        return header_score > 0

    def _build_metadata(self, data: bytes, dialect: Dialect) -> Metadata:
        data_after_preamble = self._skip_lines(data, dialect.header.num_preamble_rows)

        table = Table.parse(
            data_after_preamble,
            PotentialDialect(
                delimiter=dialect.delimiter,
                quote=dialect.quote,
                line_terminator=dialect.line_terminator,
            ),
        )

        if dialect.header.has_header_row and len(table.field_counts) > 0:
            data_field_counts = table.field_counts[1:]
        else:
            data_field_counts = table.field_counts

        is_uniform = len(data_field_counts) == 0 or all(
            count == data_field_counts[0] for count in data_field_counts
        )

        dialect.flexible = not is_uniform

        num_fields = table.get_modal_field_count()

        fields: list[str] = []
        if dialect.header.has_header_row and table.num_rows() > 0:
            header_row = table.rows[0]
            fields = [field.strip() for field in header_row] if header_row else []
        else:
            fields = [f"field_{i + 1}" for i in range(num_fields)]

        total_bytes = len(data_after_preamble)
        total_rows = table.num_rows()
        avg_record_len = total_bytes / total_rows if total_rows > 0 else 0

        return Metadata(
            dialect=dialect,
            avg_record_len=avg_record_len,
            num_fields=num_fields,
            fields=fields,
        )
