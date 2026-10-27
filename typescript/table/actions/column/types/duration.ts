import type { DurationColumn } from "@fairspec/metadata"
import { Temporal } from "temporal-polyfill"
import type { ColumnMapping } from "../../../models/column.ts"
import type { Table } from "../../../models/table.ts"
import { inspectTextColumn } from "../-shared/helpers.ts"

export async function inspectDurationColumn(
  column: DurationColumn,
  table: Table,
  mapping: ColumnMapping,
) {
  return inspectTextColumn(column, table, {
    mapping,
    // TODO: Fix this hack
    parse: parseDuration,
  })
}

function parseDuration(source: string) {
  // Temporal reads `P1DT`, though ISO 8601 needs a component after the `T`
  if (/t$/i.test(source)) {
    throw new Error(`A time designator without components: ${source}`)
  }

  return Temporal.Duration.from(source)
}
