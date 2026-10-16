from fairspec_metadata.models.column.string import StringColumn, StringColumnProperty

from ..property import (
    get_base_property_type,
    get_is_nullable_property_type,
    set_property_nullable,
)


class TestGetBasePropertyType:
    def test_returns_type_for_string(self):
        assert get_base_property_type("string") == "string"

    def test_returns_base_type_for_type_null(self):
        assert get_base_property_type(["string", "null"]) == "string"

    def test_returns_base_type_for_null_type(self):
        assert get_base_property_type(["null", "string"]) == "string"

    def test_returns_none_for_none(self):
        assert get_base_property_type(None) is None


class TestGetIsNullablePropertyType:
    def test_returns_false_for_string(self):
        assert get_is_nullable_property_type("string") is False

    def test_returns_true_for_type_null(self):
        assert get_is_nullable_property_type(["string", "null"]) is True

    def test_returns_true_for_null_type(self):
        assert get_is_nullable_property_type(["null", "string"]) is True

    def test_returns_false_for_none(self):
        assert get_is_nullable_property_type(None) is False


class TestSetPropertyNullable:
    def test_widens_type_to_a_tuple(self):
        column = StringColumn(name="name", type="string", property=StringColumnProperty())
        set_property_nullable(column)
        assert column.property.type == ("string", "null")

    def test_does_not_warn_on_model_dump(self, recwarn):
        column = StringColumn(name="name", type="string", property=StringColumnProperty())
        set_property_nullable(column)
        column.property.model_dump()
        assert len(recwarn) == 0

    def test_keeps_an_already_nullable_type(self):
        property = StringColumnProperty(type=("string", "null"))
        column = StringColumn(name="name", type="string", property=property)
        set_property_nullable(column)
        assert column.property.type == ("string", "null")
