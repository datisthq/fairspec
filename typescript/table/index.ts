export { denormalizeTable } from "./actions/table/denormalize.ts"
export { inspectTable } from "./actions/table/inspect.ts"
export { normalizeTable } from "./actions/table/normalize.ts"
export { queryTable } from "./actions/table/query.ts"
export { inferTableSchemaFromTable } from "./actions/tableSchema/infer.ts"
export { DenormalizeColumnOptions } from "./models/column.ts"
export type { Frame } from "./models/frame.ts"
export { InferTableSchemaOptions, TableSchemaOptions } from "./models/schema.ts"
export type { Table } from "./models/table.ts"
export { LoadTableOptions, SaveTableOptions } from "./models/table.ts"
export type { TablePlugin } from "./plugin.ts"

export { ArrowPlugin, loadArrowTable, saveArrowTable } from "./plugins/arrow/index.ts"
export { CsvPlugin, loadCsvTable, saveCsvTable } from "./plugins/csv/index.ts"
export { InlinePlugin, loadInlineTable } from "./plugins/inline/index.ts"
export {
  inferJsonFileDialect,
  JsonPlugin,
  loadJsonTable,
  saveJsonTable,
} from "./plugins/json/index.ts"
export {
  loadParquetTable,
  ParquetPlugin,
  saveParquetTable,
} from "./plugins/parquet/index.ts"
export { loadSqliteTable, SqlitePlugin, saveSqliteTable } from "./plugins/sqlite/index.ts"
export {
  inferXlsxFileDialect,
  loadXlsxTable,
  saveXlsxTable,
  XlsxPlugin,
} from "./plugins/xlxs/index.ts"
