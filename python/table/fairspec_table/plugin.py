from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_dataset.plugin import DatasetPlugin

if TYPE_CHECKING:
    from fairspec_metadata import Resource
    from fairspec_metadata import TableSchema

    from .models import (
        InferTableSchemaOptions,
        LoadTableOptions,
        SaveTableOptions,
        Table,
    )


class TablePlugin(DatasetPlugin):
    def load_table(
        self,
        resource: Resource,
        **options: Unpack[LoadTableOptions],
    ) -> Table | None:
        return None

    def save_table(self, table: Table, **options: Unpack[SaveTableOptions]) -> str | None:
        return None

    def infer_table_schema(
        self,
        resource: Resource,
        **options: Unpack[InferTableSchemaOptions],
    ) -> TableSchema | None:
        return None
