import os

import pytest

from ..normalize import normalize_path


class TestNormalizePath:
    @pytest.mark.parametrize(
        "path, basepath, expected",
        [
            (
                "path/to/file.txt",
                None,
                os.path.join("path", "to", "file.txt"),
            ),
            (
                "file.txt",
                "path/to",
                os.path.join("path", "to", "file.txt"),
            ),
            (
                "http://example.com/path/to/file.txt",
                None,
                "http://example.com/path/to/file.txt",
            ),
            (
                "http://example.com/path/to/file.txt?query=param",
                None,
                "http://example.com/path/to/file.txt?query=param",
            ),
            (
                "path/to/file.txt",
                "http://example.com",
                "http://example.com/path/to/file.txt",
            ),
            (
                "file.txt",
                "/absolute/path",
                os.path.relpath("/absolute/path/file.txt"),
            ),
            (
                "path/to/file.txt",
                "",
                os.path.join("path", "to", "file.txt"),
            ),
        ],
        ids=[
            "local path without basepath",
            "local path with local basepath",
            "remote path",
            "remote path with query string",
            "local path with remote basepath",
            "local path with absolute basepath",
            "path with empty basepath",
        ],
    )
    def test_valid(self, path: str, basepath: str | None, expected: str):
        assert normalize_path(path, basepath=basepath) == expected

    @pytest.mark.parametrize(
        "path, basepath",
        [
            ("/absolute/path/to/file.txt", None),
            ("../file.txt", "/folder"),
            ("../file.txt", "http://example.com/data"),
        ],
        ids=[
            "absolute path",
            "local traversed path",
            "remote traversed path",
        ],
    )
    def test_throw(self, path: str, basepath: str | None):
        with pytest.raises(Exception):
            normalize_path(path, basepath=basepath)
