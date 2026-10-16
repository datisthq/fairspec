from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from .models.data_schema import DataSchema, RenderDataSchemaOptions
    from .models.dataset import (
        ConvertDatasetFromOptions,
        ConvertDatasetToOptions,
        Dataset,
        RenderDatasetOptions,
    )
    from .models.descriptor import Descriptor
    from .models.table_schema import (
        ConvertTableSchemaFromOptions,
        ConvertTableSchemaToOptions,
        RenderTableSchemaOptions,
        TableSchema,
    )


class MetadataPlugin:
    def render_dataset_as(
        self, dataset: Dataset, options: RenderDatasetOptions
    ) -> str | None:
        return None

    def convert_dataset_to(
        self, dataset: Dataset, options: ConvertDatasetToOptions
    ) -> Descriptor | None:
        return None

    def convert_dataset_from(
        self, descriptor: Descriptor, options: ConvertDatasetFromOptions
    ) -> Dataset | None:
        return None

    def render_data_schema_as(
        self, data_schema: DataSchema, options: RenderDataSchemaOptions
    ) -> str | None:
        return None

    def render_table_schema_as(
        self, table_schema: TableSchema, options: RenderTableSchemaOptions
    ) -> str | None:
        return None

    def convert_table_schema_to(
        self, table_schema: TableSchema, options: ConvertTableSchemaToOptions
    ) -> Descriptor | None:
        return None

    def convert_table_schema_from(
        self, descriptor: Descriptor, options: ConvertTableSchemaFromOptions
    ) -> TableSchema | None:
        return None
