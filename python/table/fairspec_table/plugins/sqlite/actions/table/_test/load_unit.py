from __future__ import annotations

from fairspec_metadata import Resource, SqliteFileDialect

import pytest

from ..load import load_sqlite_table


class TestLoadSqliteTable:
    def test_should_raise_error_when_resource_path_is_not_defined(self):
        with pytest.raises(Exception, match="Resource path is not defined"):
            load_sqlite_table(
                Resource(fileDialect=SqliteFileDialect(tableName="fairspec"))
            )
