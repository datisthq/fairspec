import type { TableError } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { TableCheck } from "../../models/check.ts"
import type { SchemaMapping } from "../../models/schema.ts"
import type { Table } from "../../models/table.ts"
import { NUMBER_COLUMN_NAME } from "../../settings.ts"
import { normalizeColumns } from "../table/normalize.ts"

type CheckKind = "row" | "table" | { message: string }

interface CheckItem {
  index: number
  check: TableCheck
  kind: CheckKind
}

/**
 * Check every row of the table against each condition, or the whole table when a
 * condition reduces to one value; conditions see the values typed by the schema
 */
export async function inspectTableChecks(
  table: Table,
  checks: TableCheck[],
  options: {
    mapping?: SchemaMapping
    maxErrors: number
  },
) {
  const { mapping, maxErrors } = options
  const errors: TableError[] = []
  if (!checks.length) return errors

  const typedFrame = await createTypedTable(table, mapping).head(0).collect()
  const items: CheckItem[] = []
  for (const [index, check] of checks.entries()) {
    items.push({ index, check, kind: await inferCheckKind(check, typedFrame.schema) })
  }

  const rows = await collectFailingRows(
    table,
    items.filter(item => item.kind === "row"),
    { mapping, maxErrors },
  )

  const values = await collectTableValues(
    createTypedTable(table, mapping),
    items.filter(item => item.kind === "table"),
  )

  for (const { index, check, kind } of items) {
    const details = {
      checkName: check.name,
      ...(check.message !== undefined ? { message: check.message } : {}),
    }

    if (typeof kind === "object") {
      errors.push({ type: "check", checkName: check.name, message: kind.message })
      continue
    }

    if (kind === "table") {
      if (values[getCheckName(index)] === false) {
        errors.push({ type: "table/check", ...details })
      }
      continue
    }

    for (const row of rows) {
      if (row[getCheckName(index)] !== false) continue
      const rowNumber = Number(row[NUMBER_COLUMN_NAME])

      errors.push(
        check.columnName !== undefined
          ? {
              type: "cell/check",
              columnName: check.columnName,
              rowNumber,
              cell: String(row[getCellName(index)] ?? ""),
              ...details,
            }
          : { type: "row/check", rowNumber, ...details },
      )
    }
  }

  return errors
}

// A condition is run on two typed rows of nulls: a row check returns both and an
// aggregate returns one, which would otherwise be broadcast to fail every row
async function inferCheckKind(
  check: TableCheck,
  schema: Record<string, pl.DataType>,
): Promise<CheckKind> {
  if (check.columnName !== undefined && !(check.columnName in schema)) {
    return { message: `Column "${check.columnName}" is not found` }
  }

  const probeTable = pl
    .DataFrame({ [NUMBER_COLUMN_NAME]: [1, 2] })
    .lazy()
    .withColumns(
      ...Object.entries(schema)
        .filter(([name]) => name !== NUMBER_COLUMN_NAME)
        .map(([name, type]) => pl.lit(null).cast(type).alias(name)),
    )
    .drop(NUMBER_COLUMN_NAME)

  try {
    const name = getCheckName(0)
    const frame = await selectConditions(probeTable, [
      { index: 0, check, kind: "row" },
    ]).collect()

    const type = frame.getColumn(name).dtype
    if (!type.equals(pl.Bool)) {
      return { message: `Condition is ${type.variant} instead of Boolean` }
    }

    return frame.height === 1 ? "table" : "row"
  } catch (error) {
    return { message: formatErrorMessage(error) }
  }
}

async function collectFailingRows(
  table: Table,
  items: CheckItem[],
  options: { mapping?: SchemaMapping; maxErrors: number },
) {
  if (!items.length) return []

  const cellItems = items.filter(({ check }) => check.columnName !== undefined)
  const checkTable = withConditions(
    createTypedTable(table, options.mapping, cellItems),
    items,
  )

  const frame = await checkTable
    .filter(
      pl.anyHorizontal(
        items.map(({ index }) => pl.col(getCheckName(index)).not().fillNull(false)),
      ),
    )
    .select(
      pl.col(NUMBER_COLUMN_NAME),
      ...cellItems.map(({ index }) => pl.col(getCellName(index))),
      ...items.map(({ index }) => pl.col(getCheckName(index))),
    )
    .head(options.maxErrors)
    .collect()

  return frame.toRecords()
}

async function collectTableValues(table: Table, items: CheckItem[]) {
  const values: Record<string, unknown> = {}

  // A SQL and a polars aggregate cannot share one select, so each kind collects once
  for (const isSql of [true, false]) {
    const group = items.filter(({ check }) => isSqlCondition(check) === isSql)
    if (!group.length) continue

    const frame = await selectConditions(table, group).collect()
    Object.assign(values, frame.toRecords()[0])
  }

  return values
}

function createTypedTable(
  table: Table,
  mapping?: SchemaMapping,
  cellItems: CheckItem[] = [],
) {
  const sourceNames = new Set(mapping?.source.columns.map(column => column.name))

  const cellColumns = cellItems.map(({ index, check }) => {
    const columnName = check.columnName ?? ""
    const isSourced = !mapping || sourceNames.has(columnName)
    return (isSourced ? pl.col(columnName) : pl.lit(null)).alias(getCellName(index))
  })

  const numbered = table.withRowIndex(NUMBER_COLUMN_NAME, 1)
  if (!mapping) return numbered.withColumns(...cellColumns)

  return numbered.select(
    pl.col(NUMBER_COLUMN_NAME),
    ...Object.values(normalizeColumns(mapping)),
    ...cellColumns,
  )
}

function withConditions(table: Table, items: CheckItem[]) {
  const { sqlColumns, exprColumns } = formatConditionColumns(items)
  const result = sqlColumns.length
    ? executeQuery(table, `SELECT *, ${sqlColumns.join(", ")} FROM self`)
    : table

  return result.withColumns(...exprColumns)
}

function selectConditions(table: Table, items: CheckItem[]) {
  const { sqlColumns, exprColumns } = formatConditionColumns(items)
  if (sqlColumns.length) {
    return executeQuery(table, `SELECT ${sqlColumns.join(", ")} FROM self`)
  }

  return table.select(...exprColumns)
}

function executeQuery(table: Table, query: string) {
  const result = pl.SQLContext({ self: table }).execute(query)

  // nodejs-polars returns a frame wrapping a SQL error instead of throwing it
  if (result._ldf instanceof Error) throw result._ldf
  return result
}

function formatConditionColumns(items: CheckItem[]) {
  const sqlColumns: string[] = []
  const exprColumns: pl.Expr[] = []

  for (const { index, check } of items) {
    const name = getCheckName(index)
    if (typeof check.condition === "string") {
      sqlColumns.push(`(${check.condition}) AS "${name}"`)
    } else {
      exprColumns.push(check.condition.alias(name))
    }
  }

  return { sqlColumns, exprColumns }
}

function isSqlCondition(check: TableCheck) {
  return typeof check.condition === "string"
}

function formatErrorMessage(error: unknown) {
  const message = error instanceof Error ? error.message : String(error)
  return message.split("\n")[0] ?? message
}

function getCheckName(index: number) {
  return `fairspec:check:${index}`
}

function getCellName(index: number) {
  return `fairspec:check-cell:${index}`
}
