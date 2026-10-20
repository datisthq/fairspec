from fairspec_library import Dataset

from ..query import query_dataset_tool


class TestQueryDatasetTool:
    def test_should_join_two_resources_by_name(self):
        source = Dataset.model_validate(
            {
                "resources": [
                    {
                        "name": "users",
                        "data": [{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}],
                    },
                    {
                        "name": "posts",
                        "data": [{"id": 1, "user_id": 2}, {"id": 2, "user_id": 2}],
                    },
                ]
            }
        )

        result = query_dataset_tool(
            source,
            "SELECT users.name, COUNT(*) AS n FROM posts "
            "JOIN users ON posts.user_id = users.id GROUP BY users.name",
        )

        assert result.records == [{"name": "bob", "n": 2}]
