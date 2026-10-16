import os

import pytest

from ..basepath import get_basepath


class TestGetBasepath:
    @pytest.mark.parametrize(
        "path, expected",
        [
            (
                "http://example.com/path/to/file.txt",
                "http://example.com/path/to",
            ),
            (
                "https://example.com/path/to/file.txt",
                "https://example.com/path/to",
            ),
            (
                "https://example.com/path/to/file.txt?query=param",
                "https://example.com/path/to",
            ),
            (
                "https://example.com/path/to/file.txt#section",
                "https://example.com/path/to",
            ),
            (
                "https://example.com/path/to/",
                "https://example.com/path/to",
            ),
            (
                "https://example.com",
                "https://example.com",
            ),
            (
                "some/path/to/file.txt",
                os.path.join("some", "path", "to"),
            ),
            (
                "some/path/to/",
                os.path.join("some", "path"),
            ),
            ("file.txt", ""),
        ],
        ids=[
            "http URL with file",
            "https URL with file",
            "URL with query parameters",
            "URL with hash",
            "URL with no file",
            "URL with only domain",
            "local file path",
            "local path with no file",
            "root level file",
        ],
    )
    def test_get_basepath(self, path: str, expected: str):
        assert get_basepath(path) == expected
