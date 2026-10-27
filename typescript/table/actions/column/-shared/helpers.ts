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
import { getIsObject } from "../../../helpers/general.ts"
import type { ColumnMapping } from "../../../models/column.ts"
import type { Table } from "../../../models/table.ts"
import { NUMBER_COLUMN_NAME } from "../../../settings.ts"
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
  const frame = await collectCellFrame(table, options.mapping)

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
    }
  }

  return errors
}

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
  const constraintJsonSchema = column.property
  const frame = await collectCellFrame(table, options.mapping)

  for (const row of frame.toRecords() as any[]) {
    if (row.source === null) {
      if (!column.nullable) errors.push(createMissingError(column.name, row))
      continue
    }

    let target: Record<string, any> | undefined
    const checkCompat = column.type === "array" ? Array.isArray : getIsObject

    try {
      target = JSON.parse(row.source)
    } catch {}

    if (!target || !checkCompat(target)) {
      errors.push({
        type: "cell/type",
        cell: String(row.source),
        columnName: column.name,
        columnType: column.type,
        rowNumber: row[NUMBER_COLUMN_NAME],
      })

      continue
    }

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

    if (constraintJsonSchema) {
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

async function collectCellFrame(table: Table, mapping: ColumnMapping) {
  return await table
    .withRowIndex(NUMBER_COLUMN_NAME, 1)
    .select(
      pl.col(NUMBER_COLUMN_NAME),
      encodeNestedColumn(mapping, normalizeColumn(mapping, { keepType: true })).alias(
        "source",
      ),
      encodeNestedColumn(mapping, pl.col(mapping.source.name)).alias("cell"),
    )
    .collect()
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
