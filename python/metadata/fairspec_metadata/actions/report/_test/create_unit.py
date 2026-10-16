from fairspec_metadata.models.error.metadata import MetadataError

from ..create import create_report


class TestCreateReport:
    def test_valid_report_with_no_errors(self):
        report = create_report()
        assert report.valid is True
        assert report.errors == []

    def test_invalid_report_with_errors(self):
        errors = [
            MetadataError(type="metadata", message="error1", jsonPointer="/a"),
            MetadataError(type="metadata", message="error2", jsonPointer="/b"),
        ]
        report = create_report(errors)
        assert report.valid is False
        assert len(report.errors) == 2

    def test_max_errors_limits_errors(self):
        errors = [
            MetadataError(type="metadata", message=f"error{i}", jsonPointer=f"/{i}")
            for i in range(10)
        ]
        report = create_report(errors, max_errors=3)
        assert report.valid is False
        assert len(report.errors) == 3
