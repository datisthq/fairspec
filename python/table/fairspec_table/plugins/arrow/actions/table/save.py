from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_dataset import assert_local_path_vacant
from fairspec_metadata import TableSchema

from fairspec_table.actions.table.denormalize import denormalize_table
from fairspec_table.actions.table_schema.infer import infer_table_schema_from_table
from fairspec_table.plugins.arrow.settings import NATIVE_TYPES
from fairspec_table.settings import QUERY_ENGINE

if TYPE_CHECKING:
    from fairspec_table.models.table import SaveTableOptions, Table


def save_arrow_table(table: Table, **options: Unpack[SaveTableOptions]) -> str:
    path = options["path"]

    if not options.get("overwrite"):
        assert_local_path_vacant(path)

    table_schema = options.get("tableSchema")
    if not isinstance(table_schema, TableSchema):
        table_schema = infer_table_schema_from_table(table, **options, keepStrings=True)

    table = denormalize_table(table, table_schema, nativeTypes=NATIVE_TYPES)

    table.sink_ipc(path, engine=QUERY_ENGINE)

    return path
