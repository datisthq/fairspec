import type { DataSchema, RenderDataSchemaOptions } from "./models/dataSchema.ts"
import type {
  ConvertDatasetFromOptions,
  ConvertDatasetToOptions,
  Dataset,
  RenderDatasetOptions,
} from "./models/dataset.ts"
import type { Descriptor } from "./models/descriptor.ts"
import type {
  ConvertTableSchemaFromOptions,
  ConvertTableSchemaToOptions,
  RenderTableSchemaOptions,
  TableSchema,
} from "./models/tableSchema.ts"

export interface MetadataPlugin {
  renderDatasetAs?(dataset: Dataset, options: RenderDatasetOptions): string | undefined

  convertDatasetTo?(
    dataset: Dataset,
    options: ConvertDatasetToOptions,
  ): Descriptor | undefined

  convertDatasetFrom?(
    descriptor: Descriptor,
    options: ConvertDatasetFromOptions,
  ): Dataset | undefined

  renderDataSchemaAs?(
    dataSchema: DataSchema,
    options: RenderDataSchemaOptions,
  ): string | undefined

  renderTableSchemaAs?(
    tableSchema: TableSchema,
    options: RenderTableSchemaOptions,
  ): string | undefined

  convertTableSchemaTo?(
    tableSchema: TableSchema,
    options: ConvertTableSchemaToOptions,
  ): Descriptor | undefined

  convertTableSchemaFrom?(
    descriptor: Descriptor,
    options: ConvertTableSchemaFromOptions,
  ): TableSchema | undefined
}
