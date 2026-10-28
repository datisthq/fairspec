import type { CellError, Column, ColumnError, TableError } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { getTypeTimeZone } from "../../helpers/type.ts"
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
 * Check every column against its schema, keyed by column name in mapping order. The
 * integer columns in `plainIntegerColumns` read only the plain form, so a cell with a
 * fraction or an exponent fails their type check
 */
export async function inspectColumns(
  mappings: ColumnMapping[],
  table: Table,
  options: {
    maxErrors: number
    plainIntegerColumns?: ReadonlySet<string>
  },
) {
  const { maxErrors, plainIntegerColumns = new Set<string>() } = options
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
    const cellErrors = await inspectCellsInPolars(polarsMappings, table, {
      maxErrors,
      plainIntegerColumns,
    })
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
  const variant = getTypeTimeZone(mapping.source.type)
    ? "ZonedDatetime"
    : mapping.source.type.variant

  // TODO: Rebase on proper polars type definition when available
  // https://github.com/pola-rs/nodejs-polars/issues/372
  const compatMapping: Record<string, Column["type"][]> = {
    Bool: ["boolean"],
    Categorical: ["string"],
    Date: ["date"],
    Datetime: ["local-date-time"],
    Float32: ["number", "integer"],
    Float64: ["number", "integer"],
    Int16: ["integer", "number"],
    Int32: ["integer", "number"],
    Int64: ["integer", "number"],
    Int8: ["integer", "number"],
    List: ["list", "array"],
    String: ["unknown"],
    Struct: ["object", "geojson", "topojson"],
    Time: ["local-time", "time"],
    UInt16: ["integer", "number"],
    UInt32: ["integer", "number"],
    UInt64: ["integer", "number"],
    UInt8: ["integer", "number"],
    Utf8: ["unknown"],
    ZonedDatetime: ["date-time"],
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
  checkCellPattern,
  checkCellMinItems,
  checkCellMaxItems,
]

// A cell failing one of these has that error alone; its constraints go unchecked
const SHORT_CIRCUIT_CHECKS: unknown[] = [checkCellType, checkCellMissing]

async function inspectCellsInPolars(
  mappings: ColumnMapping[],
  table: Table,
  options: {
    maxErrors: number
    plainIntegerColumns: ReadonlySet<string>
  },
) {
  const { maxErrors, plainIntegerColumns } = options
  const errors = new Map<string, TableError[]>(
    mappings.map(mapping => [mapping.target.name, []]),
  )

  // The error columns carry a tag, not the error itself: a literal is broadcast to
  // every row, so a JSON template would materialize a full-length string column per
  // check. Templates are resolved back in JS once the frame is filtered down.
  let { checked, carried } = createCellChecks(mappings, plainIntegerColumns, {
    isSourceTypeDropped: true,
  })
  // A read with no check is no read, and it still has to report long rows and invalid UTF-8
  if (!checked.length && carried.length) {
    ;({ checked, carried } = createCellChecks(mappings, plainIntegerColumns, {
      isSourceTypeDropped: false,
    }))
  }
  if (!checked.length) return errors

  const checkTable = table
    .withRowIndex(NUMBER_COLUMN_NAME, 1)
    .select(
      pl.col(NUMBER_COLUMN_NAME),
      ...checked.flatMap(({ index, mapping, targetExpr, sourceExpr }) => [
        targetExpr.alias(getTargetName(index)),
        ...(sourceExpr ? [sourceExpr.alias(getSourceName(index))] : []),
        pl.col(mapping.source.name).alias(getCellName(index)),
      ]),
      ...carried.map(index =>
        pl.col(mappings[index]?.source.name ?? "").alias(getCellName(index)),
      ),
    )
    .withColumns(
      ...checked.map(({ index, errorExpr }) => errorExpr.alias(getErrorName(index))),
    )

  // A lazy frame caches nothing, so every column shares this one collect: a collect
  // per column would parse the whole source once for each
  const checkFrame = await checkTable
    .filter(
      pl.anyHorizontal(checked.map(({ index }) => pl.col(getErrorName(index)).neq(0))),
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
      const errorMask = row[getErrorName(index)]
      if (!errorMask) continue

      for (const [errorTag, template] of templates.entries()) {
        if (!(Number(errorMask) & (2 ** errorTag))) continue
        columnErrors.push({
          ...template,
          rowNumber: row[NUMBER_COLUMN_NAME],
          cell: String(row[getCellName(index)] ?? ""),
        })
      }
    }
  }

  return errors
}

// A column read as its own source cannot fail its type check, so that check is dropped,
// but the column stays in the read unless `isSourceTypeDropped` is off
function createCellChecks(
  mappings: ColumnMapping[],
  plainIntegerColumns: ReadonlySet<string>,
  options: { isSourceTypeDropped: boolean },
) {
  const checked: {
    index: number
    mapping: ColumnMapping
    templates: CellError[]
    targetExpr: pl.Expr
    sourceExpr?: pl.Expr
    errorExpr: pl.Expr
  }[] = []
  const carried: number[] = []

  for (const [index, mapping] of mappings.entries()) {
    if (mapping.target.type === "unknown") continue
    const targetExpr = normalizeColumn(mapping, {
      isPlain: plainIntegerColumns.has(mapping.source.name),
    })
    const sourceExpr = normalizeColumn(mapping, { keepType: true })
    const isSource = options.isSourceTypeDropped && getIsSameExpr(targetExpr, sourceExpr)
    const cellMapping = {
      source: pl.col(isSource ? getTargetName(index) : getSourceName(index)),
      target: pl.col(getTargetName(index)),
    }

    const pairs = CHECK_FUNCTIONS.flatMap(checkCell => {
      if (isSource && checkCell === checkCellType) return []
      const check = checkCell(mapping.target, cellMapping)
      return check ? [{ checkCell, check }] : []
    })
    if (!pairs.length) {
      if (isSource) carried.push(index)
      continue
    }

    // The error column is a bitmask over the checks, bit n set when check n fails, and
    // 0 when none does: a null for 0 would repeat the whole mask in its condition
    const bits = pairs.map((_, errorTag) => pl.lit(2 ** errorTag).cast(pl.UInt32))
    const zero = pl.lit(0).cast(pl.UInt32)
    const constraintExprs = pairs.flatMap(({ checkCell, check }, errorTag) =>
      SHORT_CIRCUIT_CHECKS.includes(checkCell)
        ? []
        : [
            pl
              .when(check.isErrorExpr)
              .then(bits[errorTag] ?? zero)
              .otherwise(zero),
          ],
    )
    let errorExpr = constraintExprs.length ? pl.sumHorizontal(constraintExprs) : zero
    for (const [errorTag, { checkCell, check }] of [...pairs.entries()].reverse()) {
      if (SHORT_CIRCUIT_CHECKS.includes(checkCell)) {
        errorExpr = pl
          .when(check.isErrorExpr)
          .then(bits[errorTag] ?? zero)
          .otherwise(errorExpr)
      }
    }

    const templates = pairs.map(({ check }) => check.errorTemplate)
    checked.push({
      index,
      mapping,
      templates,
      targetExpr,
      sourceExpr: isSource ? undefined : sourceExpr,
      errorExpr,
    })
  }

  return { checked, carried }
}

// Opaque functions do not serialize, and an expression holding one counts as different
function getIsSameExpr(a: pl.Expr, b: pl.Expr) {
  try {
    return a.serialize("json").equals(b.serialize("json"))
  } catch {
    return false
  }
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
