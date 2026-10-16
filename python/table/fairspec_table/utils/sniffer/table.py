from __future__ import annotations

from .potential_dialects import PotentialDialect


class Table:
    rows: list[list[str]]
    field_counts: list[int]
    _cached_modal_field_count: int | None

    def __init__(self, rows: list[list[str]], field_counts: list[int]) -> None:
        self.rows = rows
        self.field_counts = field_counts
        self._cached_modal_field_count = None

    @classmethod
    def parse(cls, data: bytes, dialect: PotentialDialect) -> Table:
        text = data.decode("utf-8", errors="replace")

        rows: list[list[str]] = []
        field_counts: list[int] = []

        delimiter_char = chr(dialect.delimiter)
        quote_char = chr(dialect.quote.char) if dialect.quote.char is not None else None

        terminator_string = {
            "CRLF": "\r\n",
            "CR": "\r",
            "LF": "\n",
        }[dialect.line_terminator]

        lines = text.split(terminator_string)

        for line in lines:
            if len(line) == 0:
                continue

            if quote_char:
                fields = _parse_quoted_line(line, delimiter_char, quote_char)
            else:
                fields = line.split(delimiter_char)

            rows.append(fields)
            field_counts.append(len(fields))

        return cls(rows, field_counts)

    def get_modal_field_count(self) -> int:
        if self._cached_modal_field_count is not None:
            return self._cached_modal_field_count

        if len(self.field_counts) == 0:
            self._cached_modal_field_count = 0
            return 0

        max_field_count = max(self.field_counts)

        if max_field_count <= 256:
            frequency = [0] * (max_field_count + 1)
            for count in self.field_counts:
                frequency[count] += 1

            max_freq = 0
            modal = 0
            for i in range(max_field_count + 1):
                if frequency[i] > max_freq:
                    max_freq = frequency[i]
                    modal = i

            self._cached_modal_field_count = modal
            return modal
        else:
            frequency: dict[int, int] = {}
            for count in self.field_counts:
                frequency[count] = frequency.get(count, 0) + 1

            max_freq = 0
            modal = 0
            for count, freq in frequency.items():
                if freq > max_freq:
                    max_freq = freq
                    modal = count

            self._cached_modal_field_count = modal
            return modal

    def is_uniform(self) -> bool:
        if len(self.field_counts) == 0:
            return True

        first = self.field_counts[0]
        return all(count == first for count in self.field_counts)

    def num_rows(self) -> int:
        return len(self.rows)


def _parse_quoted_line(
    line: str,
    delimiter: str,
    quote: str,
) -> list[str]:
    fields: list[str] = []
    current_field = ""
    in_quotes = False
    i = 0

    while i < len(line):
        char = line[i]

        if char == quote:
            if in_quotes and i + 1 < len(line) and line[i + 1] == quote:
                current_field += quote
                i += 2
            else:
                in_quotes = not in_quotes
                i += 1
        elif char == delimiter and not in_quotes:
            fields.append(current_field)
            current_field = ""
            i += 1
        else:
            current_field += char
            i += 1

    fields.append(current_field)
    return fields
