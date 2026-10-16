from ..column import get_columns


class TestGetColumns:
    def test_sets_required_for_listed_columns(self):
        columns = get_columns(
            {
                "required": ["id"],
                "properties": {
                    "id": {"type": "number"},
                    "name": {"type": "string"},
                },
            }
        )
        id_col = next(c for c in columns if c.name == "id")
        name_col = next(c for c in columns if c.name == "name")
        assert id_col.required is True
        assert name_col.required is None

    def test_sets_required_for_all_with_all_required(self):
        columns = get_columns(
            {
                "allRequired": True,
                "properties": {
                    "id": {"type": "number"},
                    "name": {"type": "string"},
                },
            }
        )
        assert all(c.required is True for c in columns)

    def test_sets_nullable_for_nullable_types(self):
        columns = get_columns(
            {
                "properties": {
                    "id": {"type": ["number", "null"]},
                },
            }
        )
        assert len(columns) == 1
        assert columns[0].name == "id"
        assert columns[0].nullable is True

    def test_sets_both_required_and_nullable(self):
        columns = get_columns(
            {
                "allRequired": True,
                "properties": {
                    "id": {"type": ["number", "null"]},
                },
            }
        )
        assert len(columns) == 1
        assert columns[0].required is True
        assert columns[0].nullable is True
