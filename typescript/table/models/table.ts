import { InferFileDialectOptions } from "@fairspec/dataset"
import type * as pl from "nodejs-polars"
import { z } from "zod"
import { InferTableSchemaOptions, TableSchemaOptions } from "./schema.ts"

export type Table = pl.LazyDataFrame

export const LoadTableOptions = InferFileDialectOptions.and(InferTableSchemaOptions).and(
  z.object({
    previewBytes: z.number().optional(),
    denormalized: z.boolean().optional(),
  }),
)

export type LoadTableOptions = z.infer<typeof LoadTableOptions>

export const SaveTableOptions = TableSchemaOptions.extend({
  path: z.string(),
  fileDialect: z.any().optional(),
  tableSchema: z.any().optional(),
  overwrite: z.boolean().optional(),
})

export type SaveTableOptions = z.infer<typeof SaveTableOptions>
