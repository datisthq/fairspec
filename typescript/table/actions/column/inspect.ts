import type { CellError, Column, ColumnError, TableError } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { ColumnMapping } from "../../models/column.ts"
import type { Table } from "../../models/table.ts"
import { NUMBER_COLUMN_NAME } from "../../settings.ts"
import { checkCellConst } from "./checks/const.ts"
import { checkCellEnum } from "./checks/enum.ts"
import { checkCellMaxItems } from "./checks/maxItems.ts"
import { createCheckCellMaximum } from "./checks/maximum.ts"
import { checkCellMaxLength } from "./checks/maxLength.ts"
import { checkCellMinItems } from "./checks/minItems.ts"
import { createCheckCellMinimum } from "./checks/minimum.ts"
import { checkCellMinLength } from "./checks/minLength.ts"
import { checkCellMissing } from "./checks/missing.ts"
import { checkCellMultipleOf } from "./checks/multipleOf.ts"
import { checkCellPattern } from "./checks/pattern.ts"
import { checkCellType } from "./checks/type.ts"
import { normalizeColumn } from "./normalize.ts"
import { inspectArrayColumn } from "./types/array.ts"
import { inspectDurationColumn } from "./types/duration.ts"
import { inspectGeojsonColumn } from "./types/geojson.ts"
import { inspectObjectColumn } from "./types/object.ts"
import { inspectTopojsonColumn } from "./types/topojson.ts"
import { inspectWkbColumn } from "./types/wkb.ts"
import { inspectWktColumn } from "./types/wkt.ts"

/**
 * Check every column against its schema, keyed by column name in mapping order
 */
export async function inspectColumns(
  mappings: ColumnMapping[],
  table: Table,
  options: {
    maxErrors: number
  },
) {
  const { maxErrors } = options
  const errors = new Map<string, TableError[]>()
  const polarsMappings: ColumnMapping[] = []

  for (const mapping of mappings) {
    const typeErrors: TableError[] = inspectType(mapping)
    errors.set(mapping.target.name, typeErrors)
    if (!typeErrors.length && !ROW_INSPECTED_TYPES.has(mapping.target.type)) {
      polarsMappings.push(mapping)
    }
  }

  if (polarsMappings.length) {
    const cellErrors = await inspectCellsInPolars(polarsMappings, table, { maxErrors })
    for (const [name, columnErrors] of cellErrors) {
      errors.set(name, columnErrors)
    }
  }

  let errorCount = 0
  for (const mapping of mappings) {
    const columnErrors = errors.get(mapping.target.name) ?? []
    const isChecked = !columnErrors.length && errorCount < maxErrors
    if (isChecked && ROW_INSPECTED_TYPES.has(mapping.target.type)) {
      columnErrors.push(...(await inspectCellsByRow(mapping, table)))
    }
    errorCount += columnErrors.length
  }

  return errors
}

function inspectType(mapping: ColumnMapping) {
  const errors: ColumnError[] = []
  const variant = mapping.source.type.variant

  // TODO: Rebase on proper polars type definition when available
  // https://github.com/pola-rs/nodejs-polars/issues/372
  const compatMapping: Record<string, Column["type"][]> = {
    Bool: ["boolean"],
    Categorical: ["string"],
    Date: ["date"],
    Datetime: ["date-time"],
    Float32: ["number", "integer"],
    Float64: ["number", "integer"],
    Int16: ["integer", "number"],
    Int32: ["integer", "number"],
    Int64: ["integer", "number"],
    Int8: ["integer", "number"],
    List: ["list"],
    String: ["unknown"],
    Time: ["time"],
    UInt16: ["integer", "number"],
    UInt32: ["integer", "number"],
    UInt64: ["integer", "number"],
    UInt8: ["integer", "number"],
    Utf8: ["unknown"],
  }

  const compatTypes = compatMapping[variant] ?? []
  const isCompat = !!new Set(compatTypes).intersection(
    new Set([mapping.target.type, "unknown"]),
  ).size

  if (!isCompat) {
    errors.push({
      type: "column/type",
      columnName: mapping.target.name,
      expectedColumnType: mapping.target.type,
      actualColumnType: compatTypes[0] ?? "unknown",
    })
  }

  return errors
}

// These types are parsed row by row outside polars, so each keeps its own scan
const ROW_INSPECTED_TYPES = new Set<Column["type"]>([
  "duration",
  "wkb",
  "wkt",
  "array",
  "object",
  "geojson",
  "topojson",
])

async function inspectCellsByRow(mapping: ColumnMapping, table: Table) {
  switch (mapping.target.type) {
    case "duration":
      return await inspectDurationColumn(mapping.target, table, mapping)
    case "wkb":
      return await inspectWkbColumn(mapping.target, table, mapping)
    case "wkt":
      return await inspectWktColumn(mapping.target, table, mapping)
    case "array":
      return await inspectArrayColumn(mapping.target, table, mapping)
    case "object":
      return await inspectObjectColumn(mapping.target, table, mapping)
    case "geojson":
      return await inspectGeojsonColumn(mapping.target, table, mapping)
    case "topojson":
      return await inspectTopojsonColumn(mapping.target, table, mapping)
    default:
      return []
  }
}

const CHECK_FUNCTIONS = [
  checkCellType,
  checkCellMissing,
  checkCellEnum,
  checkCellConst,
  createCheckCellMinimum(),
  createCheckCellMaximum(),
  createCheckCellMinimum({ isExclusive: true }),
  createCheckCellMaximum({ isExclusive: true }),
  checkCellMultipleOf,
  checkCellMinLength,
  checkCellMaxLength,
  checkCellMinItems,
  checkCellMaxItems,
  checkCellPattern,
]

async function inspectCellsInPolars(
  mappings: ColumnMapping[],
  table: Table,
  options: {
    maxErrors: number
  },
) {
  const { maxErrors } = options
  const errors = new Map<string, TableError[]>(
    mappings.map(mapping => [mapping.target.name, []]),
  )

  // The error columns carry a tag, not the error itself: a literal is broadcast to
  // every row, so a JSON template would materialize a full-length string column per
  // check. Templates are resolved back in JS once the frame is filtered down.
  const checked: {
    index: number
    mapping: ColumnMapping
    templates: CellError[]
    errorExpr: pl.Expr
  }[] = []

  for (const [index, mapping] of mappings.entries()) {
    const cellMapping = {
      source: pl.col(getSourceName(index)),
      target: pl.col(getTargetName(index)),
    }

    const checks = CHECK_FUNCTIONS.map(checkCell =>
      checkCell(mapping.target, cellMapping),
    ).filter(check => !!check)
    if (!checks.length) continue

    // The first failing check wins, so the chain is built from the last one back
    let errorExpr = pl.lit(null).cast(pl.UInt8)
    for (const [errorTag, check] of [...checks.entries()].reverse()) {
      errorExpr = pl
        .when(check.isErrorExpr)
        .then(pl.lit(errorTag).cast(pl.UInt8))
        .otherwise(errorExpr)
    }

    const templates = checks.map(check => check.errorTemplate)
    checked.push({ index, mapping, templates, errorExpr })
  }

  if (!checked.length) return errors

  const checkTable = table
    .withRowIndex(NUMBER_COLUMN_NAME, 1)
    .select(
      pl.col(NUMBER_COLUMN_NAME),
      ...checked.flatMap(({ index, mapping }) => [
        normalizeColumn(mapping).alias(getTargetName(index)),
        normalizeColumn(mapping, { keepType: true }).alias(getSourceName(index)),
        pl.col(mapping.source.name).alias(getCellName(index)),
      ]),
    )
    .withColumns(
      ...checked.map(({ index, errorExpr }) => errorExpr.alias(getErrorName(index))),
    )

  // A lazy frame caches nothing, so every column shares this one collect: a collect
  // per column would parse the whole source once for each
  const checkFrame = await checkTable
    .filter(
      pl.anyHorizontal(
        checked.map(({ index }) => pl.col(getErrorName(index)).isNotNull()),
      ),
    )
    .select(
      pl.col(NUMBER_COLUMN_NAME),
      ...checked.flatMap(({ index }) => [
        pl.col(getCellName(index)),
        pl.col(getErrorName(index)),
      ]),
    )
    .head(maxErrors)
    .collect()

  const rows = checkFrame.toRecords() as any[]
  for (const { index, mapping, templates } of checked) {
    const columnErrors = errors.get(mapping.target.name) ?? []

    for (const row of rows) {
      const errorTag = row[getErrorName(index)]
      if (errorTag === null || errorTag === undefined) continue

      columnErrors.push({
        ...(templates[errorTag] as CellError),
        rowNumber: row[NUMBER_COLUMN_NAME],
        cell: String(row[getCellName(index)] ?? ""),
      })
    }
  }

  return errors
}

function getTargetName(index: number) {
  return `fairspec:target:${index}`
}

function getSourceName(index: number) {
  return `fairspec:source:${index}`
}

function getCellName(index: number) {
  return `fairspec:cell:${index}`
}

function getErrorName(index: number) {
  return `fairspec:error:${index}`
}
