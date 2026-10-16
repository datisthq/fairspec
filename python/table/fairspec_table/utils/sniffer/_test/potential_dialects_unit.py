from __future__ import annotations

from ..metadata import LineTerminator
from ..potential_dialects import (
    DELIMITERS,
    QUOTE_CHARS,
    detect_line_terminator,
    generate_potential_dialects,
)


class TestDetectLineTerminator:
    def test_detect_lf(self):
        data = "line1\nline2\nline3".encode()
        assert detect_line_terminator(data) == LineTerminator.LF

    def test_detect_crlf(self):
        data = "line1\r\nline2\r\nline3".encode()
        assert detect_line_terminator(data) == LineTerminator.CRLF

    def test_detect_cr(self):
        data = "line1\rline2\rline3".encode()
        assert detect_line_terminator(data) == LineTerminator.CR

    def test_prefer_crlf_over_lf(self):
        data = "line1\r\nline2\nline3".encode()
        assert detect_line_terminator(data) == LineTerminator.CRLF

    def test_prefer_lf_over_cr(self):
        data = "line1\nline2\rline3".encode()
        assert detect_line_terminator(data) == LineTerminator.LF

    def test_default_lf_for_empty(self):
        assert detect_line_terminator(b"") == LineTerminator.LF

    def test_default_lf_for_single_line(self):
        data = "single line without terminator".encode()
        assert detect_line_terminator(data) == LineTerminator.LF


class TestGeneratePotentialDialects:
    def test_generates_all_combinations(self):
        dialects = generate_potential_dialects(LineTerminator.LF)
        assert len(dialects) == len(DELIMITERS) * len(QUOTE_CHARS)

    def test_uses_provided_line_terminator(self):
        dialects = generate_potential_dialects(LineTerminator.CRLF)
        assert all(d.line_terminator == "CRLF" for d in dialects)

    def test_includes_all_delimiters(self):
        dialects = generate_potential_dialects(LineTerminator.LF)
        delimiters = {d.delimiter for d in dialects}
        for delimiter in DELIMITERS:
            assert delimiter in delimiters

    def test_includes_all_quote_chars(self):
        dialects = generate_potential_dialects(LineTerminator.LF)
        quotes = [d.quote for d in dialects]

        has_none = any(q.char is None for q in quotes)
        has_double_quote = any(q.char == 34 for q in quotes)
        has_single_quote = any(q.char == 39 for q in quotes)

        assert has_none
        assert has_double_quote
        assert has_single_quote

    def test_generates_exactly_30(self):
        dialects = generate_potential_dialects(LineTerminator.LF)
        assert len(dialects) == 30


class TestDelimitersConstant:
    def test_includes_common_delimiters(self):
        assert 44 in DELIMITERS
        assert 9 in DELIMITERS
        assert 59 in DELIMITERS
        assert 124 in DELIMITERS

    def test_has_10_delimiters(self):
        assert len(DELIMITERS) == 10


class TestQuoteCharsConstant:
    def test_includes_none(self):
        assert any(q.char is None for q in QUOTE_CHARS)

    def test_includes_double_quote(self):
        assert any(q.char == 34 for q in QUOTE_CHARS)

    def test_includes_single_quote(self):
        assert any(q.char == 39 for q in QUOTE_CHARS)

    def test_has_3_options(self):
        assert len(QUOTE_CHARS) == 3
