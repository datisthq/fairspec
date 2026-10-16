import type * as pl from "nodejs-polars"
import type { PolarsSchema } from "../models/schema.ts"

export function getPolarsSchema(typeMapping: Record<string, pl.DataType>): PolarsSchema {
  const entries = Object.entries(typeMapping)
  const columns = entries.map(([name, type]) => ({ name, type }))

  return { columns }
}
