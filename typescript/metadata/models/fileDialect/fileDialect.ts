import { z } from "zod"
import { ArrowFileDialect } from "./arrow.ts"
import { CsvFileDialect } from "./csv.ts"
import { JsonFileDialect } from "./json.ts"
import { JsonlFileDialect } from "./jsonl.ts"
import { OdsFileDialect } from "./ods.ts"
import { ParquetFileDialect } from "./parquet.ts"
import { SqliteFileDialect } from "./sqlite.ts"
import { TsvFileDialect } from "./tsv.ts"
// import { UnknownFileDialect } from "./unknown.ts"
import { XlsxFileDialect } from "./xlsx.ts"

// TODO: Recovert support for unknown dialects
// Currently, it causes zod discriminated union to throw an error

export const FileDialect = z.discriminatedUnion("format", [
  CsvFileDialect,
  TsvFileDialect,
  JsonFileDialect,
  JsonlFileDialect,
  XlsxFileDialect,
  OdsFileDialect,
  SqliteFileDialect,
  ParquetFileDialect,
  ArrowFileDialect,
  // UnknownFileDialect,
])

export type FileDialect = z.infer<typeof FileDialect>
