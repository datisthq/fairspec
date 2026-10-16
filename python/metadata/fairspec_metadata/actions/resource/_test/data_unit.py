from fairspec_metadata import Resource

from ..data import (
    get_data_first_path,
    get_data_path,
    get_data_paths,
    get_data_records,
    get_data_value,
)


class TestGetDataPath:
    def test_returns_string_data(self):
        assert get_data_path(Resource(data="table.csv")) == "table.csv"

    def test_returns_list_of_strings(self):
        assert get_data_path(Resource(data=["a.csv", "b.csv"])) == ["a.csv", "b.csv"]

    def test_returns_none_for_inline_data(self):
        assert get_data_path(Resource(data=[{"id": 1}])) is None

    def test_returns_none_for_missing_data(self):
        assert get_data_path(Resource()) is None


class TestGetDataValue:
    def test_returns_inline_data(self):
        assert get_data_value(Resource(data=[{"id": 1}])) == [{"id": 1}]

    def test_returns_none_for_path(self):
        assert get_data_value(Resource(data="table.csv")) is None


class TestGetDataRecords:
    def test_returns_records_from_list(self):
        assert get_data_records(Resource(data=[{"id": 1}])) == [{"id": 1}]

    def test_returns_none_for_dict(self):
        assert get_data_records(Resource(data={"id": 1})) is None

    def test_returns_none_for_path(self):
        assert get_data_records(Resource(data="table.csv")) is None


class TestGetDataPaths:
    def test_returns_list_from_string(self):
        assert get_data_paths(Resource(data="table.csv")) == ["table.csv"]

    def test_returns_list_from_list(self):
        assert get_data_paths(Resource(data=["a.csv", "b.csv"])) == ["a.csv", "b.csv"]

    def test_returns_empty_for_missing(self):
        assert get_data_paths(Resource()) == []


class TestGetDataFirstPath:
    def test_returns_string_data(self):
        assert get_data_first_path(Resource(data="table.csv")) == "table.csv"

    def test_returns_first_from_list(self):
        assert get_data_first_path(Resource(data=["a.csv", "b.csv"])) == "a.csv"

    def test_returns_none_for_missing(self):
        assert get_data_first_path(Resource()) is None
