import type {
  Dataset,
  FileDialect,
  MetadataPlugin,
  Report,
  Resource,
} from "@fairspec/metadata"
import type { SaveDatasetOptions } from "./models/dataset.ts"
import type { InferFileDialectOptions } from "./models/fileDialect.ts"

export interface DatasetPlugin extends MetadataPlugin {
  loadDataset?(source: string): Promise<Dataset | undefined>

  /**
   * Validate a dataset source this plugin owns, reporting in the terms of its own format;
   * `validateDataset` checks the loaded dataset's resources and foreign keys
   */
  validateDataset?(
    source: string,
    options: { validateDataset: (dataset: Dataset) => Promise<Report> },
  ): Promise<Report | undefined>

  saveDataset?(
    dataset: Dataset,
    options: SaveDatasetOptions,
  ): Promise<{ path?: string } | undefined>

  inferFileDialect?(
    resource: Resource,
    options?: InferFileDialectOptions,
  ): Promise<FileDialect | undefined>
}
