import pytest
from fairspec_library import Resource, write_temp_file

from ..query import query_table_tool


class TestQueryTableTool:
    def test_should_filter_records(self):
        path = write_temp_file(
            "id,name,age\n1,alice,25\n2,bob,30\n3,carol,28\n", format="csv"
        )

        result = query_table_tool(
            Resource(data=path), "SELECT * FROM self WHERE age > 25"
        )

        assert result.records == [
            {"id": 2, "name": "bob", "age": 30},
            {"id": 3, "name": "carol", "age": 28},
        ]

    def test_should_aggregate_records(self):
        path = write_temp_file("id,score\n1,10\n2,20\n", format="csv")

        result = query_table_tool(
            Resource(data=path), "SELECT SUM(score) AS total FROM self"
        )

        assert result.records == [{"total": 30}]

    def test_should_return_json_values(self):
        path = write_temp_file("id,born\n1,2000-01-02\n2,\n", format="csv")

        result = query_table_tool(Resource(data=path), "SELECT * FROM self")

        assert result.records == [
            {"id": 1, "born": "2000-01-02"},
            {"id": 2, "born": None},
        ]

    def test_should_query_inline_data(self):
        resource = Resource(data=[{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}])

        result = query_table_tool(resource, "SELECT name FROM self WHERE id = 2")

        assert result.records == [{"name": "bob"}]

    def test_should_raise_for_unloadable_table(self):
        with pytest.raises(ValueError, match="Could not load table"):
            query_table_tool(Resource(), "SELECT * FROM self")
