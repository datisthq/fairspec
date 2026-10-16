import pytest

from ..denormalize import denormalize_path


class TestDenormalizePath:
    @pytest.mark.parametrize(
        "path, basepath, expected",
        [
            (
                "http://example.com/path/to/file.txt",
                None,
                "http://example.com/path/to/file.txt",
            ),
            (
                "http://example.com/path/to/file.txt",
                "data",
                "http://example.com/path/to/file.txt",
            ),
            (
                "/tmp/data/file.csv",
                "/tmp",
                "data/file.csv",
            ),
            (
                "/tmp/file.csv",
                "/tmp",
                "file.csv",
            ),
            (
                "/tmp/data/nested/deep/file.csv",
                "/tmp/data/nested",
                "deep/file.csv",
            ),
            (
                "/home/user/projects/data/file.csv",
                "/home/user/projects",
                "data/file.csv",
            ),
        ],
        ids=[
            "remote URL without basepath",
            "remote URL with basepath",
            "local file in subfolder",
            "local file in direct child folder",
            "local file with deeply nested basepath",
            "local file with multi-level basepath",
        ],
    )
    def test_denormalize_path(
        self,
        path: str,
        basepath: str | None,
        expected: str,
    ):
        assert denormalize_path(path, basepath=basepath) == expected
