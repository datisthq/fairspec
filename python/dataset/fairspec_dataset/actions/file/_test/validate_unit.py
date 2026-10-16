from fairspec_metadata import Integrity, Resource
from fairspec_metadata import IntegrityType

from fairspec_dataset.actions.file.temp import write_temp_file

from ..infer import infer_hash
from ..validate import validate_file


class TestValidateFile:
    def test_validates_textual_for_utf8(self):
        path = write_temp_file("Hello, World!")
        report = validate_file(Resource(data=path, textual=True))
        assert report.valid is True
        assert report.errors == []

    def test_validates_textual_for_ascii(self):
        path = write_temp_file(b"Simple ASCII text only")
        report = validate_file(Resource(data=path, textual=True))
        assert report.valid is True
        assert report.errors == []

    def test_returns_error_when_textual_expected_but_binary(self):
        path = write_temp_file(bytes([0xFF, 0xD8, 0xFF, 0xE0, 0x00]))
        report = validate_file(Resource(data=path, textual=True))
        assert report.valid is False
        assert len(report.errors) == 1
        assert report.errors[0].type == "file/textual"

    def test_returns_error_when_textual_expected_but_latin1(self):
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
        report = validate_file(Resource(data=path, textual=True))
        assert report.valid is False
        assert len(report.errors) == 1
        assert report.errors[0].type == "file/textual"

    def test_validates_integrity_md5(self):
        path = write_temp_file("Hello, World!")
        actual_hash = infer_hash(Resource(data=path), hash_type="md5")
        report = validate_file(
            Resource(
                data=path,
                integrity=Integrity(type=IntegrityType.md5, hash=actual_hash),
            )
        )
        assert report.valid is True
        assert report.errors == []

    def test_returns_error_when_integrity_mismatch(self):
        path = write_temp_file("Hello, World!")
        actual_hash = infer_hash(Resource(data=path), hash_type="md5")
        report = validate_file(
            Resource(
                data=path,
                integrity=Integrity(type=IntegrityType.md5, hash="wronghash"),
            )
        )
        assert report.valid is False
        assert len(report.errors) == 1
        assert report.errors[0].type == "file/integrity"
        assert report.errors[0].hashType == "md5"  # type: ignore[union-attr]
        assert report.errors[0].expectedHash == "wronghash"  # type: ignore[union-attr]
        assert report.errors[0].actualHash == actual_hash  # type: ignore[union-attr]

    def test_validates_sha256_integrity(self):
        path = write_temp_file("Hello, World!")
        actual_hash = infer_hash(Resource(data=path), hash_type="sha256")
        report = validate_file(
            Resource(
                data=path,
                integrity=Integrity(type=IntegrityType.sha256, hash=actual_hash),
            )
        )
        assert report.valid is True
        assert report.errors == []

    def test_validates_sha1_integrity(self):
        path = write_temp_file("Hello, World!")
        actual_hash = infer_hash(Resource(data=path), hash_type="sha1")
        report = validate_file(
            Resource(
                data=path,
                integrity=Integrity(type=IntegrityType.sha1, hash=actual_hash),
            )
        )
        assert report.valid is True
        assert report.errors == []

    def test_validates_sha512_integrity(self):
        path = write_temp_file("Hello, World!")
        actual_hash = infer_hash(Resource(data=path), hash_type="sha512")
        report = validate_file(
            Resource(
                data=path,
                integrity=Integrity(type=IntegrityType.sha512, hash=actual_hash),
            )
        )
        assert report.valid is True
        assert report.errors == []

    def test_validates_both_textual_and_integrity(self):
        path = write_temp_file("Hello, World!")
        actual_hash = infer_hash(Resource(data=path), hash_type="md5")
        report = validate_file(
            Resource(
                data=path,
                textual=True,
                integrity=Integrity(type=IntegrityType.md5, hash=actual_hash),
            )
        )
        assert report.valid is True
        assert report.errors == []

    def test_returns_multiple_errors(self):
        path = write_temp_file(bytes([0xFF, 0xD8, 0xFF, 0xE0, 0x00]))
        report = validate_file(
            Resource(
                data=path,
                textual=True,
                integrity=Integrity(type=IntegrityType.md5, hash="wronghash"),
            )
        )
        assert report.valid is False
        assert len(report.errors) == 2
        assert report.errors[0].type == "file/textual"
        assert report.errors[1].type == "file/integrity"

    def test_returns_error_only_textual_mismatch(self):
        path = write_temp_file(bytes([0xFF, 0xD8, 0xFF, 0xE0, 0x00]))
        actual_hash = infer_hash(Resource(data=path), hash_type="md5")
        report = validate_file(
            Resource(
                data=path,
                textual=True,
                integrity=Integrity(type=IntegrityType.md5, hash=actual_hash),
            )
        )
        assert report.valid is False
        assert len(report.errors) == 1
        assert report.errors[0].type == "file/textual"

    def test_returns_error_only_integrity_mismatch(self):
        path = write_temp_file("Hello, World!")
        report = validate_file(
            Resource(
                data=path,
                textual=True,
                integrity=Integrity(type=IntegrityType.md5, hash="wronghash"),
            )
        )
        assert report.valid is False
        assert len(report.errors) == 1
        assert report.errors[0].type == "file/integrity"

    def test_handles_empty_file(self):
        path = write_temp_file("")
        actual_hash = infer_hash(Resource(data=path), hash_type="sha256")
        report = validate_file(
            Resource(
                data=path,
                integrity=Integrity(type=IntegrityType.sha256, hash=actual_hash),
            )
        )
        assert report.valid is True
        assert report.errors == []

    def test_validates_textual_with_special_characters(self):
        path = write_temp_file("Special: é, ñ, ü, ö, à")
        report = validate_file(Resource(data=path, textual=True))
        assert report.valid is True
        assert report.errors == []
