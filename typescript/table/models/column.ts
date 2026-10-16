import type { Column } from "@fairspec/metadata"
import type * as pl from "nodejs-polars"
import { z } from "zod"

export type PolarsColumn = {
  name: string
  type: pl.DataType
}

export interface ColumnMapping {
  source: PolarsColumn
  target: Column
}

export const DenormalizeColumnOptions = z.object({
  nativeTypes: z.array(z.string()).optional(),
})

export type DenormalizeColumnOptions = z.infer<typeof DenormalizeColumnOptions>
