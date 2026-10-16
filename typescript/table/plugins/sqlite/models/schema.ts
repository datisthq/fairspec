import type { TableMetadata } from "kysely"

export interface SqliteSchema extends TableMetadata {
  primaryKey?: string[]
}
