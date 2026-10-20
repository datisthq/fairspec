import * as pl from "nodejs-polars"
import type { Table } from "../../models/table.ts"

/** Run a SQL query against a table, exposed as `self`. */
export function queryTable(table: Table, query: string) {
  return queryTables({ self: table }, query)
}

/** Run a SQL query against several tables, each exposed under its key. */
export function queryTables(tables: Record<string, Table>, query: string) {
  const context = pl.SQLContext(tables)
  return context.execute(query)
}
