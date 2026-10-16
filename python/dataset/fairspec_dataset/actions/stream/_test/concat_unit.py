from io import BytesIO

from ..concat import concat_file_streams


class TestConcatFileStreams:
    def test_concatenates_multiple_streams_in_order(self):
        stream1 = BytesIO(b"Hello, ")
        stream2 = BytesIO(b"World")
        stream3 = BytesIO(b"!")

        result = concat_file_streams([stream1, stream2, stream3])
        assert result.read() == b"Hello, World!"

    def test_handles_single_stream(self):
        stream = BytesIO(b"Single stream content")

        result = concat_file_streams([stream])
        assert result.read() == b"Single stream content"

    def test_handles_empty_array(self):
        result = concat_file_streams([])
        assert result.read() == b""

    def test_handles_streams_with_empty_content(self):
        stream1 = BytesIO(b"")
        stream2 = BytesIO(b"Content")
        stream3 = BytesIO(b"")

        result = concat_file_streams([stream1, stream2, stream3])
        assert result.read() == b"Content"

    def test_concatenates_streams_with_multiple_chunks(self):
        stream1 = BytesIO(b"ABC")
        stream2 = BytesIO(b"DEF")

        result = concat_file_streams([stream1, stream2])
        assert result.read() == b"ABCDEF"

    def test_handles_binary_data(self):
        stream1 = BytesIO(bytes([0x00, 0x01, 0x02]))
        stream2 = BytesIO(bytes([0x03, 0x04, 0x05]))

        result = concat_file_streams([stream1, stream2])
        assert result.read() == bytes([0x00, 0x01, 0x02, 0x03, 0x04, 0x05])

    def test_handles_large_streams(self):
        content1 = b"A" * 5000
        content2 = b"B" * 5000
        stream1 = BytesIO(content1)
        stream2 = BytesIO(content2)

        result = concat_file_streams([stream1, stream2])
        data = result.read()
        assert len(data) == 10000
        assert data == content1 + content2

    def test_preserves_unicode_characters(self):
        stream1 = BytesIO("Hello 世界".encode())
        stream2 = BytesIO(" مرحبا".encode())
        stream3 = BytesIO(" 🌍".encode())

        result = concat_file_streams([stream1, stream2, stream3])
        assert result.read().decode() == "Hello 世界 مرحبا 🌍"

    def test_maintains_stream_order_with_many_streams(self):
        streams = [BytesIO(str(i).encode()) for i in range(10)]

        result = concat_file_streams(streams)
        assert result.read() == b"0123456789"
