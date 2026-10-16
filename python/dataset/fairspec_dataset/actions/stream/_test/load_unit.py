import pytest

from fairspec_dataset.actions.file.temp import write_temp_file

from ..load import load_file_stream


class TestLoadFileStream:
    def test_loads_stream_from_single_local_file(self):
        path = write_temp_file("Hello, World!")
        stream = load_file_stream(path)
        assert stream.read() == b"Hello, World!"

    def test_loads_stream_from_array_using_default_index(self):
        file1 = write_temp_file("First file content")
        file2 = write_temp_file("Second file content")
        stream = load_file_stream([file1, file2])
        assert stream.read() == b"First file content"

    def test_loads_stream_from_array_using_specified_index(self):
        file1 = write_temp_file("First file content")
        file2 = write_temp_file("Second file content")
        stream = load_file_stream([file1, file2], index=1)
        assert stream.read() == b"Second file content"

    def test_limits_stream_to_max_bytes(self):
        path = write_temp_file("This is a long content that should be truncated")
        stream = load_file_stream(path, max_bytes=10)
        data = stream.read()
        assert data == b"This is a "
        assert len(data) == 10

    def test_raises_error_for_invalid_index(self):
        path = write_temp_file("content")
        with pytest.raises(ValueError, match="Cannot stream resource"):
            load_file_stream([path], index=5)

    def test_raises_error_for_empty_array(self):
        with pytest.raises(ValueError, match="Cannot stream resource"):
            load_file_stream([], index=0)

    def test_handles_large_files(self):
        content = "A" * 10000
        path = write_temp_file(content)
        stream = load_file_stream(path)
        data = stream.read()
        assert len(data) == 10000
        assert data == content.encode()

    def test_handles_binary_content(self):
        binary_data = bytes([0x00, 0x01, 0x02, 0x03, 0xFF])
        path = write_temp_file(binary_data)
        stream = load_file_stream(path)
        assert stream.read() == binary_data

    def test_handles_empty_files(self):
        path = write_temp_file("")
        stream = load_file_stream(path)
        assert stream.read() == b""

    def test_limits_bytes_correctly(self):
        path = write_temp_file("0123456789ABCDEFGHIJ")
        stream = load_file_stream(path, max_bytes=5)
        assert stream.read() == b"01234"

    def test_handles_max_bytes_larger_than_file(self):
        path = write_temp_file("Short")
        stream = load_file_stream(path, max_bytes=1000)
        assert stream.read() == b"Short"
