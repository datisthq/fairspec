import os

from ..create import create_folder


class TestCreateFolder:
    def test_creates_simple_folder(self, tmp_path):
        path = str(tmp_path / "test-folder")
        create_folder(path)
        assert os.path.isdir(path)

    def test_creates_nested_folders(self, tmp_path):
        path = str(tmp_path / "parent" / "child" / "grandchild")
        create_folder(path)
        assert os.path.isdir(path)
        assert os.path.isdir(str(tmp_path / "parent"))
        assert os.path.isdir(str(tmp_path / "parent" / "child"))

    def test_no_error_when_folder_exists(self, tmp_path):
        path = str(tmp_path / "existing")
        create_folder(path)
        create_folder(path)
        assert os.path.isdir(path)

    def test_creates_deeply_nested_directories(self, tmp_path):
        path = str(tmp_path / "l1" / "l2" / "l3" / "l4" / "l5")
        create_folder(path)
        assert os.path.isdir(path)

    def test_creates_folder_with_special_characters(self, tmp_path):
        path = str(tmp_path / "folder-with_special.chars")
        create_folder(path)
        assert os.path.isdir(path)

    def test_creates_sibling_folders(self, tmp_path):
        for name in ("folder1", "folder2", "folder3"):
            create_folder(str(tmp_path / name))
        for name in ("folder1", "folder2", "folder3"):
            assert os.path.isdir(str(tmp_path / name))

    def test_creates_complex_directory_structure(self, tmp_path):
        dirs = [
            str(tmp_path / "project" / "src" / "components"),
            str(tmp_path / "project" / "src" / "utils"),
            str(tmp_path / "project" / "tests" / "unit"),
            str(tmp_path / "project" / "tests" / "integration"),
        ]
        for d in dirs:
            create_folder(d)
        for d in dirs:
            assert os.path.isdir(d)
