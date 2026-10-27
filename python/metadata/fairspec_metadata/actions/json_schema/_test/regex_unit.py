import pytest

from ..regex import is_valid_ecma_regex


class TestIsValidEcmaRegex:
    @pytest.mark.parametrize(
        "pattern",
        [
            r"^[A-Z]{3}-[0-9]{4}$",
            r"^(?<year>\d{4})-\k<year>$",
            r"^\p{L}+$",
            r"\u{1F600}",
            r"[^]",
            r"(?=a)b(?<!c)",
            r"(a)\1",
            r"(?i:abc)",
            r"[\w.\-]+\@",
            r"😀",
            r"(?<x>a)|(?<x>b)",
        ],
    )
    def test_accepts(self, pattern: str):
        assert is_valid_ecma_regex(pattern)

    @pytest.mark.parametrize(
        "pattern",
        [
            "[a-",
            "(?i)abc",
            "(?P<n>x)",
            "a{,3}",
            "a{3,1}",
            r"\Z",
            "(a",
            "a)",
            "*a",
            r"\1",
            r"\k<y>",
            "(?<x>a)(?<x>b)",
            "[z-a]",
            r"\u{110000}",
        ],
    )
    def test_rejects(self, pattern: str):
        assert not is_valid_ecma_regex(pattern)
