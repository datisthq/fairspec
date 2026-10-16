from __future__ import annotations

from typing import TYPE_CHECKING, Unpack, cast

from fairspec_metadata import Resource, get_supported_file_dialect
from fairspec_metadata.models.file_dialect.file_dialect import FileDialect

from fairspec_table.plugin import TablePlugin

from .actions.file_dialect.infer import infer_xlsx_file_dialect
from .actions.table.load import load_xlsx_table
from .actions.table.save import save_xlsx_table

if TYPE_CHECKING:
    from fairspec_dataset.models.file_dialect import InferFileDialectOptions

    from fairspec_table.models.table import LoadTableOptions, SaveTableOptions, Table


class XlsxPlugin(TablePlugin):
    def load_table(
        self,
        resource: Resource,
        **options: Unpack[LoadTableOptions],
    ) -> Table | None:
        file_dialect = get_supported_file_dialect(resource, ["xlsx", "ods"])
        if not file_dialect:
            return None

        return load_xlsx_table(
            resource.model_copy(update={"fileDialect": file_dialect}), **options
        )

    def save_table(self, table: Table, **options: Unpack[SaveTableOptions]) -> str | None:
        resource = Resource(
            data=options["path"],
            fileDialect=cast(FileDialect | None, options.get("fileDialect")),
        )
        file_dialect = get_supported_file_dialect(resource, ["xlsx", "ods"])
        if not file_dialect:
            return None
        return save_xlsx_table(table, **options)

    def infer_file_dialect(
        self,
        resource: Resource,
        **options: Unpack[InferFileDialectOptions],
    ) -> FileDialect | None:
        file_dialect = get_supported_file_dialect(resource, ["xlsx", "ods"])
        if not file_dialect:
            return None
        return infer_xlsx_file_dialect(resource)
