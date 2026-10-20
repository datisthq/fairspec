import type { TableSchema } from "@fairspec/metadata"
import type { Random } from "../generate/random.ts"

export type Profile = {
  name: string
  description: string
  tableSchema: TableSchema
  columns: Column[]
  faults: Fault[]
}

export type Column = {
  name: string
  /** Returns the cell text; an empty string is a null. */
  generate: (random: Random, row: number) => string
}

/** One injected bad cell, which must break exactly one check. */
export type Fault = {
  column: string
  check: Check
  value: string
}

export type Check = "type" | "required" | "range" | "enum" | "pattern"
