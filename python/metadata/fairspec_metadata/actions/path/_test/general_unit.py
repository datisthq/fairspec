import os

import pytest

from ..general import (
    get_file_extension,
    get_file_name,
    get_file_name_slug,
    get_is_remote_path,
)


class TestGetIsRemotePath:
    @pytest.mark.parametrize(
        "path, expected",
        [
            ("http://example.com/path/to/file.txt", True),
            ("https://example.com/path/to/file.txt", True),
            ("ftp://example.com/path/to/file.txt", True),
            ("file:///path/to/file.txt", False),
            ("/path/to/file.txt", False),
            ("path/to/file.txt", False),
            ("./file.txt", False),
            ("../file.txt", False),
            ("", False),
            ("http:example.com", True),
        ],
        ids=[
            "http URL",
            "https URL",
            "ftp URL",
            "file URL",
            "absolute path",
            "relative path",
            "current directory path",
            "parent directory path",
            "empty string",
            "protocol without slashes",
        ],
    )
    def test_get_is_remote_path(self, path: str, expected: bool):
        assert get_is_remote_path(path) == expected


class TestGetFileName:
    @pytest.mark.parametrize(
        "path, expected",
        [
            ("file.txt", "file.txt"),
            ("some/path/to/file.txt", "file.txt"),
            (
                "http://example.com/path/to/file.txt",
                "file.txt",
            ),
            (
                "https://example.com/path/to/file.txt",
                "file.txt",
            ),
            (
                "https://example.com/path/to/file.txt?query=param",
                "file.txt",
            ),
            (
                "https://example.com/path/to/file.txt#section",
                "file.txt",
            ),
            (
                "https://example.com/path/to/file.txt?query=param#section",
                "file.txt",
            ),
            ("https://example.com/path/", None),
            (f"some{os.sep}path{os.sep}", None),
        ],
        ids=[
            "simple filename",
            "directory path with filename",
            "remote HTTP URL",
            "remote HTTPS URL",
            "URL with query parameters",
            "URL with hash",
            "URL with query and hash",
            "URL with no filename",
            "local path with no filename",
        ],
    )
    def test_get_file_name(self, path: str, expected: str | None):
        assert get_file_name(path) == expected


class TestGetFileExtension:
    def test_infers_format_from_single_string_path(self):
        assert get_file_extension("/data/users.csv") == "csv"

    def test_infers_format_from_url_path(self):
        assert get_file_extension("https://example.com/data/products.json") == "json"

    def test_preserve_extension_case(self):
        assert get_file_extension("/data/file.CSV") == "CSV"

    def test_returns_format_name_even_for_unsupported_extensions(
        self,
    ):
        assert get_file_extension("/data/file.tar.gz") == "gz"

    def test_returns_none_when_path_has_no_extension(self):
        assert get_file_extension("/data/file") is None

    def test_returns_none_when_filename_cannot_be_determined(
        self,
    ):
        assert get_file_extension("/data/folder/") is None

    def test_handles_multiple_extensions(self):
        assert get_file_extension("/data/file.backup.csv") == "csv"

    def test_handles_hidden_files_with_extension(self):
        assert get_file_extension("/data/.gitignore") is None

    def test_handles_url_with_query_parameters(self):
        assert get_file_extension("https://example.com/file.json?key=value") == "json"

    def test_handles_url_with_hash(self):
        assert get_file_extension("https://example.com/file.pdf#page=1") == "pdf"


class TestGetFileNameSlug:
    def test_returns_slugified_basename_from_single_string_path(
        self,
    ):
        assert get_file_name_slug("/data/users.csv") == "users"

    def test_returns_slugified_basename_from_url_path(self):
        assert get_file_name_slug("https://example.com/data/products.json") == "products"

    def test_returns_none_when_path_has_no_filename(self):
        assert get_file_name_slug("/data/folder/") is None

    def test_handles_complex_filename_with_multiple_dots(self):
        assert get_file_name_slug("/data/file.backup.csv") == "file_backup"

    def test_slugifies_filename_with_spaces_and_special_characters(
        self,
    ):
        assert get_file_name_slug("/data/My Data File!.csv") == "my_data_file"

    def test_returns_none_for_empty_string(self):
        assert get_file_name_slug("") is None

    def test_handles_simple_filename_without_directory(self):
        assert get_file_name_slug("document.txt") == "document"

    def test_handles_url_with_query_parameters(self):
        assert get_file_name_slug("https://example.com/file.json?key=value") == "file"

    def test_handles_url_with_hash(self):
        assert get_file_name_slug("https://example.com/report.pdf#page=1") == "report"

    def test_handles_hidden_files(self):
        assert get_file_name_slug("/data/.gitignore") == "gitignore"

    def test_slugifies_uppercase_letters_to_lowercase(self):
        assert get_file_name_slug("/data/MyDocument.PDF") == "my_document"

    def test_replaces_hyphens_with_underscores(self):
        assert get_file_name_slug("/data/my-file-name.csv") == "my_file_name"
