import type { DurationColumn } from "@fairspec/metadata"
import type { ColumnMapping } from "../../../models/column.ts"
import type { Table } from "../../../models/table.ts"
import { DURATION_REGEX } from "../../../settings.ts"
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

const DURATION_PATTERN = new RegExp(DURATION_REGEX)

function parseDuration(source: string) {
  if (!DURATION_PATTERN.test(source)) {
    throw new Error(`Not an RFC 3339 duration: ${source}`)
  }

  return source
}
