import pytest

from ..basepath import get_common_local_basepath


class TestGetCommonLocalBasepath:
    @pytest.mark.parametrize(
        "description, paths, expected",
        [
            (
                "same directory different files",
                ["data/table1.csv", "data/table2.csv"],
                "data",
            ),
            (
                "nested directories",
                ["data/nested/file1.csv", "data/nested/file2.csv", "data/file3.csv"],
                "data",
            ),
            (
                "single path",
                ["data/file.csv"],
                "data",
            ),
            (
                "root level files",
                ["file1.csv", "file2.csv"],
                "",
            ),
            (
                "different top-level directories",
                ["data1/file1.csv", "data2/file2.csv"],
                "",
            ),
            (
                "empty paths array",
                [],
                None,
            ),
            (
                "some paths are remote",
                ["https://example.com/table.csv", "data/table.csv"],
                "data",
            ),
            (
                "all paths are remote",
                [
                    "https://example.com/table1.csv",
                    "https://example.com/table2.csv",
                ],
                None,
            ),
        ],
    )
    def test_get_common_local_basepath(self, description, paths, expected):
        result = get_common_local_basepath(paths)
        if expected is None:
            assert result is None
        else:
            assert result == expected
