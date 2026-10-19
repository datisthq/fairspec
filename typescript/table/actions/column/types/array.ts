import type { ArrayColumn } from "@fairspec/metadata"
import type { ColumnMapping } from "../../../models/column.ts"
import type { Table } from "../../../models/table.ts"
import { inspectJsonColumn } from "../-shared/helpers.ts"

export async function inspectArrayColumn(
  column: ArrayColumn,
  table: Table,
  mapping: ColumnMapping,
) {
  return inspectJsonColumn(column, table, { mapping })
}
