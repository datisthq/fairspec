from __future__ import annotations

import polars as pl
import pytest
from fairspec_dataset import get_temp_file_path
from fairspec_metadata import Resource, SqliteFileDialect

from fairspec_table.plugins.sqlite.actions.table.save import save_sqlite_table

from ..infer import infer_table_schema_from_sqlite

DIALECT = SqliteFileDialect(tableName="fairspec")


class TestInferTableSchemaFromSqlite:
    def test_should_infer_schema(self):
        path = get_temp_file_path()

        source = pl.DataFrame(
            [
                pl.Series("string", ["string"], dtype=pl.String),
                pl.Series("integer", [1], dtype=pl.Int32),
                pl.Series("number", [1.1], dtype=pl.Float64),
            ]
        ).lazy()

        save_sqlite_table(source, path=path, fileDialect=DIALECT, overwrite=True)

        schema = infer_table_schema_from_sqlite(Resource(data=path, fileDialect=DIALECT))
        properties = {
            name: prop.model_dump(exclude_none=True)
            for name, prop in schema["properties"].items()
        }

        assert schema["required"] == []
        assert schema["primaryKey"] is None
        assert properties == {
            "string": {"type": ("string", "null")},
            "integer": {"type": ("integer", "null")},
            "number": {"type": ("number", "null")},
        }

    def test_should_raise_error_when_resource_path_is_not_defined(self):
        with pytest.raises(Exception, match="Database is not defined"):
            infer_table_schema_from_sqlite(
                Resource(fileDialect=SqliteFileDialect(tableName="fairspec"))
            )
