import os
from io import BytesIO

import pytest

from ..save import save_file_stream


class TestSaveFileStream:
    def test_saves_stream_to_file(self, tmp_path):
        path = str(tmp_path / "test.txt")
        save_file_stream(BytesIO(b"Hello, World!"), path=path)
        assert os.path.exists(path)
        with open(path, "rb") as f:
            assert f.read() == b"Hello, World!"

    def test_saves_stream_with_multiple_chunks(self, tmp_path):
        path = str(tmp_path / "chunks.txt")
        save_file_stream(BytesIO(b"Hello, World!"), path=path)
        with open(path, "rb") as f:
            assert f.read() == b"Hello, World!"

    def test_creates_nested_directories(self, tmp_path):
        path = str(tmp_path / "nested" / "dir" / "file.txt")
        save_file_stream(BytesIO(b"Nested content"), path=path)
        with open(path, "rb") as f:
            assert f.read() == b"Nested content"

    def test_raises_when_file_exists_and_overwrite_false(self, tmp_path):
        path = str(tmp_path / "existing.txt")
        save_file_stream(BytesIO(b"Initial content"), path=path)
        with pytest.raises(FileExistsError):
            save_file_stream(BytesIO(b"New content"), path=path, overwrite=False)

    def test_raises_when_file_exists_and_overwrite_not_specified(self, tmp_path):
        path = str(tmp_path / "existing2.txt")
        save_file_stream(BytesIO(b"Initial content"), path=path)
        with pytest.raises(FileExistsError):
            save_file_stream(BytesIO(b"New content"), path=path)

    def test_overwrites_when_overwrite_true(self, tmp_path):
        path = str(tmp_path / "overwrite.txt")
        save_file_stream(BytesIO(b"Initial content"), path=path)
        save_file_stream(BytesIO(b"New content"), path=path, overwrite=True)
        with open(path, "rb") as f:
            assert f.read() == b"New content"

    def test_saves_binary_data(self, tmp_path):
        binary_data = bytes([0x00, 0x01, 0x02, 0x03, 0xFF])
        path = str(tmp_path / "binary.bin")
        save_file_stream(BytesIO(binary_data), path=path)
        with open(path, "rb") as f:
            assert f.read() == binary_data

    def test_saves_empty_stream(self, tmp_path):
        path = str(tmp_path / "empty.txt")
        save_file_stream(BytesIO(b""), path=path)
        with open(path, "rb") as f:
            assert f.read() == b""

    def test_saves_large_stream(self, tmp_path):
        content = b"A" * 10000
        path = str(tmp_path / "large.txt")
        save_file_stream(BytesIO(content), path=path)
        with open(path, "rb") as f:
            assert f.read() == content

    def test_saves_unicode_characters(self, tmp_path):
        content = "Unicode: 你好世界 🌍 مرحبا".encode()
        path = str(tmp_path / "unicode.txt")
        save_file_stream(BytesIO(content), path=path)
        with open(path, "rb") as f:
            assert f.read() == content
