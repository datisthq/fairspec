import type { TopojsonColumn } from "@fairspec/metadata"
import type { ColumnMapping } from "../../../models/column.ts"
import type { Table } from "../../../models/table.ts"
import topojsonJsonSchema from "../../../schemas/topojson.json" with { type: "json" }
import { inspectJsonColumn } from "../-shared/helpers.ts"

export async function inspectTopojsonColumn(
  column: TopojsonColumn,
  table: Table,
  mapping: ColumnMapping,
) {
  return inspectJsonColumn(column, table, {
    mapping,
    typeJsonSchema: topojsonJsonSchema,
  })
}
