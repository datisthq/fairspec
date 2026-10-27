import type {
  ArrayColumn,
  CellError,
  DurationColumn,
  GeojsonColumn,
  ObjectColumn,
  TopojsonColumn,
  WkbColumn,
  WktColumn,
} from "@fairspec/metadata"
import { inspectJson } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { ColumnMapping } from "../../../models/column.ts"
import type { Table } from "../../../models/table.ts"
import { NUMBER_COLUMN_NAME } from "../../../settings.ts"
import { checkCellConst } from "../checks/const.ts"
import { checkCellEnum } from "../checks/enum.ts"
import { checkCellMaxLength } from "../checks/maxLength.ts"
import { checkCellMinLength } from "../checks/minLength.ts"
import { checkCellPattern } from "../checks/pattern.ts"
import { normalizeColumn } from "../normalize.ts"

// TODO: Generalize wkt/wkb/duration inspectors

export async function inspectTextColumn(
  column: DurationColumn | WktColumn | WkbColumn,
  table: Table,
  options: {
    mapping: ColumnMapping
    parse: (source: string) => unknown | undefined
  },
) {
  const errors: CellError[] = []
  const sourceExpr = pl.col("source")
  const checks = TEXT_CHECK_FUNCTIONS.flatMap(checkCell => {
    const check = checkCell(column, { source: sourceExpr, target: sourceExpr })
    return check ? [check] : []
  })
  const frame = await selectCellFrame(table, options.mapping)
    .withColumns(
      ...checks.map((check, index) => check.isErrorExpr.alias(`check:${index}`)),
    )
    .collect()

  for (const row of frame.toRecords() as any[]) {
    if (row.source === null) {
      if (!column.nullable) errors.push(createMissingError(column.name, row))
      continue
    }

    let target: unknown | undefined
    try {
      target = options.parse(row.source)
    } catch {}

    if (!target) {
      errors.push({
        type: "cell/type",
        cell: String(row.source),
        columnName: column.name,
        columnType: column.type,
        rowNumber: row[NUMBER_COLUMN_NAME],
      })
      continue
    }

    for (const [index, check] of checks.entries()) {
      if (!row[`check:${index}`]) continue
      errors.push({
        ...check.errorTemplate,
        rowNumber: row[NUMBER_COLUMN_NAME],
        cell: String(row.cell ?? ""),
      })
    }
  }

  return errors
}

// The string constraints in the spec's keyword order, checked on a cell its type reads
const TEXT_CHECK_FUNCTIONS = [
  checkCellEnum,
  checkCellConst,
  checkCellMinLength,
  checkCellMaxLength,
  checkCellPattern,
]

export async function inspectJsonColumn(
  column: ArrayColumn | ObjectColumn | GeojsonColumn | TopojsonColumn,
  table: Table,
  options: {
    mapping: ColumnMapping
    typeJsonSchema?: Record<string, any>
  },
) {
  const errors: CellError[] = []

  const typeJsonSchema = options.typeJsonSchema
  // The native kind check already holds what `type` would assert on every value
  const { type: _type, ...constraintJsonSchema } = column.property
  const needsValidator =
    !!typeJsonSchema ||
    Object.entries(constraintJsonSchema).some(
      ([key, value]) => value !== undefined && !ANNOTATION_KEYWORDS.includes(key),
    )

  let cellFrame = selectCellFrame(table, options.mapping).withColumns(
    createJsonKindExpr(options.mapping, pl.col("source")).alias("valid"),
  )
  if (!needsValidator) {
    cellFrame = cellFrame.filter(
      pl
        .col("source")
        .isNull()
        .and(pl.lit(!column.nullable))
        .or(pl.col("source").isNotNull().and(pl.col("valid").not())),
    )
  }
  const frame = await cellFrame.collect()

  for (const row of frame.toRecords() as any[]) {
    if (row.source === null) {
      if (!column.nullable) errors.push(createMissingError(column.name, row))
      continue
    }

    if (!row.valid) {
      errors.push({
        type: "cell/type",
        cell: String(row.source),
        columnName: column.name,
        columnType: column.type,
        rowNumber: row[NUMBER_COLUMN_NAME],
      })

      continue
    }

    const target = JSON.parse(row.source)
    if (typeJsonSchema) {
      const formatErrors = await inspectJson(target, {
        jsonSchema: typeJsonSchema,
      })

      if (formatErrors.length) {
        errors.push({
          type: "cell/type",
          cell: String(row.source),
          columnName: column.name,
          columnType: column.type,
          rowNumber: row[NUMBER_COLUMN_NAME],
        })
      }

      continue
    }

    if (needsValidator) {
      const constraintErrors = await inspectJson(target, {
        jsonSchema: constraintJsonSchema,
      })

      for (const error of constraintErrors) {
        errors.push({
          type: "cell/json",
          cell: String(row.source),
          columnName: column.name,
          rowNumber: row[NUMBER_COLUMN_NAME],
          message: error.message,
          jsonPointer: error.jsonPointer,
        })
      }
    }
  }

  return errors
}

// Keywords that never fail a value: a column with only these needs no validator
const ANNOTATION_KEYWORDS = [
  "type",
  "title",
  "description",
  "examples",
  "default",
  "$comment",
  "deprecated",
  "readOnly",
  "writeOnly",
  "rdfType",
  "missingValues",
]

function selectCellFrame(table: Table, mapping: ColumnMapping) {
  return table
    .withRowIndex(NUMBER_COLUMN_NAME, 1)
    .select(
      pl.col(NUMBER_COLUMN_NAME),
      encodeNestedColumn(mapping, normalizeColumn(mapping, { keepType: true })).alias(
        "source",
      ),
      encodeNestedColumn(mapping, pl.col(mapping.source.name)).alias("cell"),
    )
}

// Polars' parser is the one judge of a JSON cell, so every library rejects the same text
function createJsonKindExpr(mapping: ColumnMapping, sourceExpr: pl.Expr) {
  if (["Struct", "List"].includes(mapping.source.type.variant)) return pl.lit(true)
  const opener =
    mapping.target.type === "array"
      ? String.raw`^[ \t\n\r]*\[`
      : String.raw`^[ \t\n\r]*\{`
  return sourceExpr.str
    .contains(opener)
    .and(sourceExpr.str.jsonPathMatch("$").isNotNull())
}

// Wrapping in a one-field struct lets List encode too, which has no jsonEncode
function encodeNestedColumn(mapping: ColumnMapping, columnExpr: pl.Expr) {
  if (!["Struct", "List"].includes(mapping.source.type.variant)) return columnExpr
  const encoded = pl.struct([columnExpr.alias("v")]).struct.jsonEncode()
  return pl
    .when(columnExpr.isNotNull())
    .then(encoded.str.replace(String.raw`(?s)^\{"v":(.*)\}$`, "${1}"))
    .otherwise(pl.lit(null))
}

function createMissingError(columnName: string, row: Record<string, unknown>) {
  const error: CellError = {
    type: "cell/missing",
    columnName,
    rowNumber: Number(row[NUMBER_COLUMN_NAME]),
    cell: String(row.cell ?? ""),
  }

  return error
}
