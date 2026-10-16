import { z } from "zod"
import { ColumnProperty } from "./column/column.ts"
import { ForeignKey } from "./foreignKey.ts"
import { UniqueKey } from "./uniqueKey.ts"

export const TableSchema = z.object({
  $schema: z.httpUrl().optional().describe("Fairspec Schema profile url."),

  title: z.string().optional().describe("A human-readable title of the table schema"),

  description: z
    .string()
    .optional()
    .describe("A human-readable description of the table schema"),

  required: z
    .array(z.string())
    .optional()
    .describe("An optional list of column names that must be present"),

  allRequired: z
    .boolean()
    .optional()
    .describe("An optional boolean indicating whether all columns are required"),

  properties: z
    .record(z.string(), ColumnProperty)
    .optional()
    .describe(
      "An object defining the schema for table columns, where each key is a column name",
    ),

  missingValues: z
    .array(
      z.union([
        z.string(),
        z.int(),
        z.number(),
        z.object({
          value: z.union([z.string(), z.int(), z.number()]),
          label: z.string(),
        }),
      ]),
    )
    .optional()
    .describe(
      "An optional list of values that represent missing or null data across all columns",
    ),

  primaryKey: z
    .array(z.string())
    .min(1)
    .optional()
    .describe("An optional array of column names that form the table's primary key"),

  uniqueKeys: z
    .array(UniqueKey)
    .min(1)
    .optional()
    .describe("An optional array of unique key constraints"),

  foreignKeys: z
    .array(ForeignKey)
    .min(1)
    .optional()
    .describe("An optional array of foreign key constraints"),
})

export const RenderTableSchemaOptions = z.object({
  format: z.string(),
})

export const ConvertTableSchemaToOptions = z.object({
  format: z.string(),
})

export const ConvertTableSchemaFromOptions = z.object({
  format: z.string(),
})

export type TableSchema = z.infer<typeof TableSchema>
export type RenderTableSchemaOptions = z.infer<typeof RenderTableSchemaOptions>
export type ConvertTableSchemaToOptions = z.infer<typeof ConvertTableSchemaToOptions>
export type ConvertTableSchemaFromOptions = z.infer<typeof ConvertTableSchemaFromOptions>
