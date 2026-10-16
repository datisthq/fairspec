import { z } from "zod"
import { BaseFileDialect } from "./base.ts"

export const ParquetFileDialect = BaseFileDialect.extend({
  format: z.literal("parquet"),
})

export type ParquetFileDialect = z.infer<typeof ParquetFileDialect>
