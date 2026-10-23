import pytest

from ..parse import Error, parse_descriptor


class TestParseDescriptor:
    def test_parses_object(self):
        assert parse_descriptor('{"name": "test"}') == {"name": "test"}

    @pytest.mark.parametrize(
        "text, kind",
        [
            ('["secret", "token"]', "array"),
            ('"secret token"', "string"),
            ("42", "number"),
            ("1.5", "number"),
            ("true", "boolean"),
            ("null", "null"),
        ],
    )
    def test_rejects_non_object_without_echoing_input(self, text, kind):
        with pytest.raises(Error) as info:
            parse_descriptor(text)
        assert (
            str(info.value) == f"Invalid descriptor: expected a JSON object, got {kind}"
        )
