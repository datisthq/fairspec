from __future__ import annotations

from ..quote import quote_identifier


class TestQuoteIdentifier:
    def test_should_wrap_a_name_in_double_quotes(self):
        assert quote_identifier("name") == '"name"'

    def test_should_double_embedded_double_quotes(self):
        assert quote_identifier('a"b') == '"a""b"'
