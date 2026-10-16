from fairspec_metadata import Resource

from fairspec_dataset.actions.file.temp import write_temp_file

from ..infer import infer_bytes, infer_hash, infer_textual


class TestInferHash:
    def test_computes_sha256_hash_by_default(self):
        path = write_temp_file("Hello, World!")
        result = infer_hash(Resource(data=path))
        assert len(result) == 64

    def test_computes_md5_hash(self):
        path = write_temp_file("Hello, World!")
        result = infer_hash(Resource(data=path), hash_type="md5")
        assert len(result) == 32

    def test_computes_sha1_hash(self):
        path = write_temp_file("Hello, World!")
        result = infer_hash(Resource(data=path), hash_type="sha1")
        assert len(result) == 40

    def test_computes_sha512_hash(self):
        path = write_temp_file("Hello, World!")
        result = infer_hash(Resource(data=path), hash_type="sha512")
        assert len(result) == 128

    def test_consistent_hashes_for_same_content(self):
        path = write_temp_file("Hello, World!")
        result1 = infer_hash(Resource(data=path))
        result2 = infer_hash(Resource(data=path))
        assert result1 == result2


class TestInferBytes:
    def test_returns_file_size(self):
        path = write_temp_file("Hello, World!")
        result = infer_bytes(Resource(data=path))
        assert result == 13

    def test_handles_empty_files(self):
        path = write_temp_file("")
        result = infer_bytes(Resource(data=path))
        assert result == 0

    def test_handles_larger_files(self):
        path = write_temp_file("x" * 10000)
        result = infer_bytes(Resource(data=path))
        assert result == 10000

    def test_handles_binary_data(self):
        path = write_temp_file(bytes([0xFF, 0xD8, 0xFF, 0xE0]))
        result = infer_bytes(Resource(data=path))
        assert result == 4


class TestInferTextual:
    def test_returns_true_for_utf8_text(self):
        path = write_temp_file("Hello, World! This is UTF-8 text.")
        assert infer_textual(Resource(data=path)) is True

    def test_returns_false_for_binary(self):
        path = write_temp_file(bytes([0xFF, 0xD8, 0xFF, 0xE0, 0x00]))
        assert infer_textual(Resource(data=path)) is False

    def test_uses_custom_sample_bytes(self):
        path = write_temp_file("This is a test file with UTF-8 content.")
        assert infer_textual(Resource(data=path), sample_bytes=20) is True

    def test_handles_large_text_files(self):
        path = write_temp_file("Hello World! " * 1000)
        assert infer_textual(Resource(data=path)) is True

    def test_handles_empty_files(self):
        path = write_temp_file("")
        assert infer_textual(Resource(data=path)) is True

    def test_handles_special_characters(self):
        path = write_temp_file("Special: é, ñ, ü, ö, à")
        assert infer_textual(Resource(data=path)) is True

    def test_returns_true_for_ascii(self):
        path = write_temp_file(b"Simple ASCII text only")
        assert infer_textual(Resource(data=path)) is True

    def test_returns_true_for_utf8_with_unicode(self):
        path = write_temp_file("Héllo, Wörld! 你好 مرحبا 🌍")
        assert infer_textual(Resource(data=path)) is True

    def test_returns_true_for_cyrillic(self):
        path = write_temp_file("Привет мир")
        assert infer_textual(Resource(data=path)) is True

    def test_returns_true_for_japanese(self):
        path = write_temp_file("こんにちは世界")
        assert infer_textual(Resource(data=path)) is True

    def test_returns_true_for_arabic(self):
        path = write_temp_file("مرحبا بالعالم")
        assert infer_textual(Resource(data=path)) is True

    def test_returns_false_for_latin1(self):
        buffer = bytes(
            [
                0x43,
                0x61,
                0x66,
                0xE9,
                0x20,
                0x72,
                0xE9,
                0x73,
                0x75,
                0x6D,
                0xE9,
                0x20,
                0x6E,
                0x61,
                0xEF,
                0x76,
                0x65,
                0x20,
                0xE0,
                0x20,
                0x50,
                0x61,
                0x72,
                0x69,
                0x73,
                0x2E,
                0x20,
                0xC7,
                0x61,
                0x20,
                0x63,
                0x27,
                0x65,
                0x73,
                0x74,
                0x20,
                0x62,
                0x6F,
                0x6E,
                0x21,
            ]
        )
        path = write_temp_file(buffer)
        assert infer_textual(Resource(data=path)) is False

    def test_returns_false_for_windows_1252(self):
        buffer = bytes(
            [
                0x43,
                0x61,
                0x66,
                0xE9,
                0x20,
                0x6E,
                0x61,
                0xEF,
                0x76,
                0x65,
                0x20,
                0x72,
                0xE9,
                0x73,
                0x75,
                0x6D,
                0xE9,
            ]
        )
        path = write_temp_file(buffer)
        assert infer_textual(Resource(data=path)) is False


class TestInferHashConcurrency:
    def test_hashes_multipart_identically_under_concurrency(self):
        paths = [write_temp_file(f"part-{index}\n") for index in range(4)]
        resource = Resource(data=paths)

        serial = infer_hash(resource, concurrency=1)
        concurrent = infer_hash(resource, concurrency=4)

        assert serial == concurrent
        assert len(serial) == 64

    def test_hashes_multipart_in_path_order(self):
        first = write_temp_file("a")
        second = write_temp_file("b")

        forward = infer_hash(Resource(data=[first, second]), concurrency=4)
        backward = infer_hash(Resource(data=[second, first]), concurrency=4)

        assert forward != backward
