import type * as pl from "nodejs-polars"
import { z } from "zod"

/**
 * A named condition every row must satisfy, beyond what the Table Schema expresses.
 * A condition reducing to one value checks the whole table instead.
 */
export const TableCheck = z.object({
  name: z.string().describe("The name of the check, reported with its errors"),
  condition: z.string().describe("A polars SQL condition; false fails, null passes"),
  columnName: z
    .string()
    .optional()
    .describe("The column whose cell a failing row is reported against"),
  message: z.string().optional().describe("The message reported with each error"),
})

// The schema keeps to what JSON carries, so MCP can publish it; code may pass a
// polars expression as well
export type TableCheck = Omit<z.infer<typeof TableCheck>, "condition"> & {
  condition: string | pl.Expr
}
