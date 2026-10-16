from ..create import create_column_from_property


class TestCreateColumnFromProperty:
    def test_creates_string_column(self):
        column = create_column_from_property("name", {"type": "string"})
        assert column.type == "string"
        assert column.nullable is None

    def test_creates_nullable_column_for_type_null(self):
        column = create_column_from_property("name", {"type": ["string", "null"]})
        assert column.type == "string"
        assert column.nullable is True

    def test_creates_nullable_column_for_null_type(self):
        column = create_column_from_property("name", {"type": ["null", "string"]})
        assert column.type == "string"
        assert column.nullable is True

    def test_creates_nullable_date_column(self):
        column = create_column_from_property(
            "created", {"type": ["string", "null"], "format": "date"}
        )
        assert column.type == "date"
        assert column.nullable is True

    def test_creates_integer_column(self):
        column = create_column_from_property("id", {"type": "integer"})
        assert column.type == "integer"

    def test_creates_boolean_column(self):
        column = create_column_from_property("flag", {"type": "boolean"})
        assert column.type == "boolean"

    def test_creates_number_column(self):
        column = create_column_from_property("value", {"type": "number"})
        assert column.type == "number"

    def test_creates_array_column(self):
        column = create_column_from_property("items", {"type": "array"})
        assert column.type == "array"

    def test_creates_object_column(self):
        column = create_column_from_property("meta", {"type": "object"})
        assert column.type == "object"

    def test_creates_geojson_column(self):
        column = create_column_from_property(
            "geo", {"type": "object", "format": "geojson"}
        )
        assert column.type == "geojson"

    def test_creates_topojson_column(self):
        column = create_column_from_property(
            "topo", {"type": "object", "format": "topojson"}
        )
        assert column.type == "topojson"

    def test_creates_categorical_column_from_string(self):
        column = create_column_from_property(
            "cat", {"type": "string", "format": "categorical"}
        )
        assert column.type == "categorical"

    def test_creates_categorical_column_from_integer(self):
        column = create_column_from_property(
            "cat", {"type": "integer", "format": "categorical"}
        )
        assert column.type == "categorical"

    def test_creates_unknown_column_for_none_type(self):
        column = create_column_from_property("x", {})
        assert column.type == "unknown"

    def test_creates_email_column(self):
        column = create_column_from_property(
            "email", {"type": "string", "format": "email"}
        )
        assert column.type == "email"

    def test_creates_url_column(self):
        column = create_column_from_property("url", {"type": "string", "format": "url"})
        assert column.type == "url"
