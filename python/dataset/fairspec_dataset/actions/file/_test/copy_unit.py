import os

from fairspec_dataset.actions.file.temp import write_temp_file

from ..copy import copy_file


class TestCopyFile:
    def test_copies_file(self, tmp_path):
        source = write_temp_file("test content")
        target = str(tmp_path / "target.txt")
        copy_file(source_path=source, target_path=target)
        assert os.path.exists(target)
        with open(target, encoding="utf-8") as f:
            assert f.read() == "test content"

    def test_copies_exact_content(self, tmp_path):
        content = "Hello, World! This is a test file."
        source = write_temp_file(content)
        target = str(tmp_path / "copy.txt")
        copy_file(source_path=source, target_path=target)
        with open(target, encoding="utf-8") as f:
            assert f.read() == content

    def test_copies_binary_file(self, tmp_path):
        binary_data = bytes([0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10])
        source = write_temp_file(binary_data)
        target = str(tmp_path / "binary.bin")
        copy_file(source_path=source, target_path=target)
        with open(target, "rb") as f:
            assert f.read() == binary_data

    def test_copies_empty_file(self, tmp_path):
        source = write_temp_file("")
        target = str(tmp_path / "empty.txt")
        copy_file(source_path=source, target_path=target)
        with open(target, encoding="utf-8") as f:
            assert f.read() == ""

    def test_copies_large_file(self, tmp_path):
        content = "x" * 100000
        source = write_temp_file(content)
        target = str(tmp_path / "large.txt")
        copy_file(source_path=source, target_path=target)
        with open(target, encoding="utf-8") as f:
            assert f.read() == content

    def test_copies_special_characters(self, tmp_path):
        content = "Special characters: é, ñ, ü, ö, à, 中文, 日本語"
        source = write_temp_file(content)
        target = str(tmp_path / "special.txt")
        copy_file(source_path=source, target_path=target)
        with open(target, encoding="utf-8") as f:
            assert f.read() == content

    def test_copies_to_nested_directory(self, tmp_path):
        source = write_temp_file("nested content")
        target = str(tmp_path / "nested" / "dir" / "file.txt")
        copy_file(source_path=source, target_path=target)
        with open(target, encoding="utf-8") as f:
            assert f.read() == "nested content"

    def test_copies_with_newlines(self, tmp_path):
        content = "Line 1\nLine 2\nLine 3\n"
        source = write_temp_file(content)
        target = str(tmp_path / "multiline.txt")
        copy_file(source_path=source, target_path=target)
        with open(target, encoding="utf-8") as f:
            assert f.read() == content
