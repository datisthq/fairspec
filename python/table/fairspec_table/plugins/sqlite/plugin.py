from __future__ import annotations

from typing import TYPE_CHECKING, Unpack, cast

from fairspec_metadata import Resource, SqliteFileDialect, get_supported_file_dialect
from fairspec_metadata.models.file_dialect.file_dialect import FileDialect

from fairspec_table.plugin import TablePlugin

from .actions.table.load import load_sqlite_table
from .actions.table.save import save_sqlite_table

if TYPE_CHECKING:
    from fairspec_dataset.models.file_dialect import InferFileDialectOptions

    from fairspec_table.models.table import LoadTableOptions, SaveTableOptions, Table


class SqlitePlugin(TablePlugin):
    def load_table(
        self,
        resource: Resource,
        **options: Unpack[LoadTableOptions],
    ) -> Table | None:
        file_dialect = get_supported_file_dialect(resource, ["sqlite"])
        if not file_dialect:
            return None
        return load_sqlite_table(resource, **options)

    def save_table(self, table: Table, **options: Unpack[SaveTableOptions]) -> str | None:
        resource = Resource(
            data=options["path"],
            fileDialect=cast(FileDialect | None, options.get("fileDialect")),
        )
        file_dialect = get_supported_file_dialect(resource, ["sqlite"])
        if not file_dialect:
            return None
        return save_sqlite_table(table, **options)

    def infer_file_dialect(
        self,
        resource: Resource,
        **options: Unpack[InferFileDialectOptions],
    ) -> FileDialect | None:
        file_dialect = get_supported_file_dialect(resource, ["sqlite"])
        if not file_dialect:
            return None
        return SqliteFileDialect()
