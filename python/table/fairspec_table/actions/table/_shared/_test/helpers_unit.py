from __future__ import annotations

from fairspec_metadata import StringColumn, StringColumnProperty
from fairspec_metadata import TableSchema, TableSchemaMissingValueItem

from ..helpers import merge_missing_values


class TestMergeMissingValues:
    def test_no_missing_values_in_schema(self):
        column = StringColumn(
            name="name",
            type="string",
            property=StringColumnProperty(),
        )
        table_schema = TableSchema()

        result = merge_missing_values(column, table_schema)

        assert result is column

    def test_merge_string_missing_values(self):
        column = StringColumn(
            name="name",
            type="string",
            property=StringColumnProperty(),
        )
        table_schema = TableSchema(missingValues=["", "NA", "N/A"])

        result = merge_missing_values(column, table_schema)

        assert result.property.missingValues == ["", "NA", "N/A"]
        assert result is not column

    def test_merge_with_existing_column_missing_values(self):
        column = StringColumn(
            name="name",
            type="string",
            property=StringColumnProperty(missingValues=["custom"]),
        )
        table_schema = TableSchema(missingValues=["", "NA"])

        result = merge_missing_values(column, table_schema)

        assert result.property.missingValues == ["custom", "", "NA"]

    def test_does_not_mutate_original_column(self):
        column = StringColumn(
            name="name",
            type="string",
            property=StringColumnProperty(missingValues=["original"]),
        )
        table_schema = TableSchema(missingValues=["added"])

        merge_missing_values(column, table_schema)

        assert column.property.missingValues == ["original"]

    def test_skip_integer_missing_values_for_string_columns(self):
        column = StringColumn(
            name="name",
            type="string",
            property=StringColumnProperty(),
        )
        table_schema = TableSchema(missingValues=["", 99])

        result = merge_missing_values(column, table_schema)

        assert result.property.missingValues == [""]

    def test_skip_integer_missing_value_item_for_string_columns(self):
        column = StringColumn(
            name="name",
            type="string",
            property=StringColumnProperty(),
        )
        table_schema = TableSchema(
            missingValues=[
                "",
                TableSchemaMissingValueItem(value=99, label="missing"),
            ]
        )

        result = merge_missing_values(column, table_schema)

        assert result.property.missingValues == [""]
