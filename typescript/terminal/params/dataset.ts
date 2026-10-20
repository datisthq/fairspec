import { f } from "fireargs"
import { z } from "zod"

export const dataset = f
  .option({ short: "d" })
  .schema(z.string().optional())
  .describe("dataset to select resource from")

export const resource = f
  .option({ short: "r" })
  .schema(z.string().optional())
  .describe("resource in provided dataset")

export const datasetQuery = f
  .argument()
  .string()
  .describe(
    "a SQL query to execute against a dataset (refer to each table by its resource name)",
  )
