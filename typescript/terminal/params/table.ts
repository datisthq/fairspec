import { f } from "fireargs"
import { z } from "zod"

export const tableQuery = f
  .argument()
  .schema(z.string().optional())
  .describe("a SQL query to execute against a table (use `self` to refer to the table)")

export const check = f
  .option({ helpGroup: "Checks:" })
  .schema(z.array(z.string().regex(/^[^=]+=/, 'Expected "name=condition"')).optional())
  .describe(
    "a named SQL condition every row must satisfy, as name=condition (repeatable)",
  )
