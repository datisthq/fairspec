import json

import pytest

from ..save import save_descriptor


class TestSaveDescriptor:
    def test_save_basic(self, tmp_path):
        path = str(tmp_path / "output.json")
        save_descriptor({"name": "test"}, path=path)
        with open(path, encoding="utf-8") as f:
            assert json.load(f) == {"name": "test"}

    def test_save_creates_nested_dirs(self, tmp_path):
        path = str(tmp_path / "a" / "b" / "output.json")
        save_descriptor({"name": "nested"}, path=path)
        with open(path, encoding="utf-8") as f:
            assert json.load(f) == {"name": "nested"}

    def test_save_uses_two_space_indent(self, tmp_path):
        path = str(tmp_path / "output.json")
        save_descriptor({"a": 1}, path=path)
        with open(path, encoding="utf-8") as f:
            text = f.read()
        assert '  "a": 1' in text

    def test_save_exclusive_fails_on_existing(self, tmp_path):
        path = str(tmp_path / "output.json")
        save_descriptor({"a": 1}, path=path)
        with pytest.raises(FileExistsError):
            save_descriptor({"a": 2}, path=path)

    def test_save_overwrite_succeeds_on_existing(self, tmp_path):
        path = str(tmp_path / "output.json")
        save_descriptor({"a": 1}, path=path)
        save_descriptor({"a": 2}, path=path, overwrite=True)
        with open(path, encoding="utf-8") as f:
            assert json.load(f) == {"a": 2}

    def test_save_file_content_is_valid_json(self, tmp_path):
        path = str(tmp_path / "output.json")
        descriptor = {"key": "value", "nested": {"a": [1, 2, 3]}}
        save_descriptor(descriptor, path=path)
        with open(path, encoding="utf-8") as f:
            loaded = json.load(f)
        assert loaded == descriptor
