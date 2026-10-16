import pytest
from fairspec_metadata import Resource

from ..save import SaveFileProps, save_resource_files


def _identity_save(props: SaveFileProps) -> str:
    return props.denormalized_path


class TestSaveResourceFiles:
    @pytest.mark.parametrize(
        "description, basepath, resource, expected, with_remote, without_folders",
        [
            (
                "local path",
                "data",
                {
                    "data": "data/table.csv",
                    "dataSchema": "data/data-schema.json",
                    "tableSchema": "data/table-schema.json",
                },
                {
                    "data": "table.csv",
                    "dataSchema": "data-schema.json",
                    "tableSchema": "table-schema.json",
                },
                False,
                False,
            ),
            (
                "local paths",
                "data",
                {"data": ["data/table1.csv", "data/table2.csv"]},
                {"data": ["table1.csv", "table2.csv"]},
                False,
                False,
            ),
            (
                "local path and remote path",
                "data",
                {
                    "data": "data/table.csv",
                    "tableSchema": "https://example.com/schema.json",
                },
                {
                    "data": "table.csv",
                    "tableSchema": "https://example.com/schema.json",
                },
                False,
                False,
            ),
            (
                "local path and remote path using with_remote",
                "data",
                {
                    "data": "data/table.csv",
                    "tableSchema": "https://example.com/schema.json",
                },
                {
                    "data": "table.csv",
                    "tableSchema": "schema.json",
                },
                True,
                False,
            ),
            (
                "remote paths with same filename using with_remote",
                "data",
                {
                    "data": [
                        "http://example1.com/table.csv",
                        "http://example2.com/table.csv",
                        "http://example3.com/table.csv",
                        "http://example4.com/table.csv.zip",
                        "http://example5.com/table.csv.zip",
                    ],
                },
                {
                    "data": [
                        "table.csv",
                        "table-1.csv",
                        "table-2.csv",
                        "table.csv.zip",
                        "table-1.csv.zip",
                    ],
                },
                True,
                False,
            ),
            (
                "local paths in different folders",
                "data",
                {
                    "data": "data/folder1/table.csv",
                    "tableSchema": "data/folder2/schema.json",
                },
                {
                    "data": "folder1/table.csv",
                    "tableSchema": "folder2/schema.json",
                },
                False,
                False,
            ),
            (
                "local paths in different folders using without_folders",
                "data",
                {
                    "data": "data/folder1/table.csv",
                    "tableSchema": "data/folder2/schema.json",
                },
                {
                    "data": "folder1-table.csv",
                    "tableSchema": "folder2-schema.json",
                },
                False,
                True,
            ),
        ],
    )
    def test_save_resource_files(
        self,
        description,
        basepath,
        resource,
        expected,
        with_remote,
        without_folders,
    ):
        result = save_resource_files(
            Resource(**resource),
            basepath=basepath,
            with_remote=with_remote,
            without_folders=without_folders,
            save_file=_identity_save,
        )
        assert result == expected
