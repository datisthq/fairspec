import pytest

from fairspec_dataset.actions.file.temp import write_temp_file

from ..preview import preview_file


def _create_lines(count: int) -> list[str]:
    return [f"{i},row{i}\n" for i in range(count)]


class TestPreviewFile:
    def test_returns_small_file_whole(self):
        path = write_temp_file("id,name\n1,alice\n")
        preview = preview_file(path)
        assert preview.text == "id,name\n1,alice\n"
        assert preview.truncated is False

    def test_keeps_first_20_lines_by_default(self):
        lines = _create_lines(50)
        path = write_temp_file("".join(lines))
        preview = preview_file(path)
        assert preview.text == "".join(lines[:20])
        assert preview.truncated is True

    def test_cuts_at_max_lines(self):
        lines = _create_lines(10)
        path = write_temp_file("".join(lines))
        preview = preview_file(path, max_lines=3)
        assert preview.text == "".join(lines[:3])
        assert preview.truncated is True

    def test_cuts_at_max_bytes_before_max_lines(self):
        path = write_temp_file("a" * 100)
        preview = preview_file(path, max_bytes=10)
        assert preview.text == "a" * 10
        assert preview.truncated is True

    def test_file_of_exactly_max_bytes_is_not_truncated(self):
        path = write_temp_file("a" * 10)
        preview = preview_file(path, max_bytes=10)
        assert preview.text == "a" * 10
        assert preview.truncated is False

    def test_file_of_exactly_max_lines_is_not_truncated(self):
        lines = _create_lines(3)
        path = write_temp_file("".join(lines))
        preview = preview_file(path, max_lines=3)
        assert preview.text == "".join(lines)
        assert preview.truncated is False

    def test_drops_multi_byte_character_cut_by_max_bytes(self):
        path = write_temp_file("a" * 99 + "é")
        preview = preview_file(path, max_bytes=100)
        assert preview.text == "a" * 99
        assert preview.truncated is True

    def test_rejects_non_positive_limits(self):
        path = write_temp_file("id\n")
        with pytest.raises(ValueError, match="max_bytes must be a positive integer"):
            preview_file(path, max_bytes=0)
        with pytest.raises(ValueError, match="max_lines must be a positive integer"):
            preview_file(path, max_lines=-1)

    def test_rejects_binary_file(self):
        path = write_temp_file(bytes([0x00, 0x01, 0x02, 0x03, 0xFF, 0x00]))
        with pytest.raises(ValueError, match="Cannot preview a binary file"):
            preview_file(path)
