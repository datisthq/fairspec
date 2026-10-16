import type { TableSchema } from "@fairspec/metadata"
import { z } from "zod"
import type { PolarsColumn } from "./column.ts"

export interface PolarsSchema {
  columns: PolarsColumn[]
}

export interface SchemaMapping {
  source: PolarsSchema
  target: TableSchema
}

export const TableSchemaOptions = z.object({
  columnNames: z.array(z.string()).optional(),
  columnTypes: z.record(z.string(), z.string()).optional(),
  missingValues: z.array(z.string()).optional(),
  decimalChar: z.string().optional(),
  groupChar: z.string().optional(),
  trueValues: z.array(z.string()).optional(),
  falseValues: z.array(z.string()).optional(),
  datetimeFormat: z.string().optional(),
  dateFormat: z.string().optional(),
  timeFormat: z.string().optional(),
  arrayType: z.enum(["array", "list"]).optional(),
  listDelimiter: z.string().optional(),
  listItemType: z
    .enum(["string", "number", "boolean", "date", "date-time", "integer", "time"])
    .optional(),
})

export type TableSchemaOptions = z.infer<typeof TableSchemaOptions>

export const InferTableSchemaOptions = TableSchemaOptions.extend({
  sampleRows: z.number().optional(),
  confidence: z.number().optional(),
  commaDecimal: z.boolean().optional(),
  monthFirst: z.boolean().optional(),
  keepStrings: z.boolean().optional(),
})

export type InferTableSchemaOptions = z.infer<typeof InferTableSchemaOptions>
