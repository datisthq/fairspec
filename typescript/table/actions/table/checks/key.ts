import type { RowPrimaryKeyError, RowUniqueKeyError } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { SchemaMapping } from "../../../models/schema.ts"

type KeyCheckType = "primary" | "unique"

export function createRowKeyChecks(mapping: SchemaMapping) {
  const names = new Set(mapping.source.columns.map(column => column.name))
  const isPresent = (key: string[]) => key.every(name => names.has(name))
  const uniqueKeys = (mapping.target.uniqueKeys ?? []).filter(isPresent)
  const primaryKey = mapping.target.primaryKey

  const checks = [
    ...(primaryKey && isPresent(primaryKey)
      ? [createRowKeyCheck(primaryKey, { keyType: "primary" })]
      : []),
    ...uniqueKeys.map(key => createRowKeyCheck(key, { keyType: "unique" })),
  ]

  return checks
}

function createRowKeyCheck(keyColumns: string[], options: { keyType: KeyCheckType }) {
  // pl.struct takes plain strings as literals, not column names
  const isErrorExpr = pl
    .struct(keyColumns.map(name => pl.col(name)))
    .isFirstDistinct()
    .not()
    .and(createKeyPresenceExpr(keyColumns))

  const errorTemplate: RowPrimaryKeyError | RowUniqueKeyError = {
    type: options.keyType === "primary" ? "row/primaryKey" : "row/uniqueKey",
    columnNames: keyColumns,
    rowNumber: 0,
  }

  return { keyColumns, isErrorExpr, errorTemplate }
}

/** A key with any null value is never a duplicate. */
export function createKeyPresenceExpr(keyColumns: string[]) {
  return pl.allHorizontal(keyColumns.map(name => pl.col(name).isNotNull()))
}
