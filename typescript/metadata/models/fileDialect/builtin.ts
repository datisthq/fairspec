import { z } from "zod"
import { ArrowFileDialect } from "./arrow.ts"
import { CsvFileDialect } from "./csv.ts"
import { JsonFileDialect } from "./json.ts"
import { JsonlFileDialect } from "./jsonl.ts"
import { OdsFileDialect } from "./ods.ts"
import { ParquetFileDialect } from "./parquet.ts"
import { SqliteFileDialect } from "./sqlite.ts"
import { TsvFileDialect } from "./tsv.ts"
import { XlsxFileDialect } from "./xlsx.ts"

export const BuiltinFileDialect = z.discriminatedUnion("format", [
  CsvFileDialect,
  TsvFileDialect,
  JsonFileDialect,
  JsonlFileDialect,
  XlsxFileDialect,
  OdsFileDialect,
  SqliteFileDialect,
  ParquetFileDialect,
  ArrowFileDialect,
])

export type BuiltinFileDialect = z.infer<typeof BuiltinFileDialect>

export const BUILTIN_FILE_DIALECT_FORMATS: string[] = BuiltinFileDialect.options.map(
  option => option.shape.format.value,
)
