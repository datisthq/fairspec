import type { WktColumn } from "@fairspec/metadata"
import { Geometry } from "wkx-ts"
import type { ColumnMapping } from "../../../models/column.ts"
import type { Table } from "../../../models/table.ts"
import { inspectTextColumn } from "../-shared/helpers.ts"

export async function inspectWktColumn(
  column: WktColumn,
  table: Table,
  mapping: ColumnMapping,
) {
  return inspectTextColumn(column, table, {
    mapping,
    parse: source => Geometry.parse(source),
  })
}
