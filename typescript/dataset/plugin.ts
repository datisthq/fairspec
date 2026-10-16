import type { Dataset, FileDialect, MetadataPlugin, Resource } from "@fairspec/metadata"
import type { SaveDatasetOptions } from "./models/dataset.ts"
import type { InferFileDialectOptions } from "./models/fileDialect.ts"

export interface DatasetPlugin extends MetadataPlugin {
  loadDataset?(source: string): Promise<Dataset | undefined>

  saveDataset?(
    dataset: Dataset,
    options: SaveDatasetOptions,
  ): Promise<{ path?: string } | undefined>

  inferFileDialect?(
    resource: Resource,
    options?: InferFileDialectOptions,
  ): Promise<FileDialect | undefined>
}
