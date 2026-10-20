import type { RowPrimaryKeyError, RowUniqueKeyError } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { SchemaMapping } from "../../../models/schema.ts"

type KeyCheckType = "primary" | "unique"

export function createRowKeyChecks(mapping: SchemaMapping) {
  const uniqueKeys = mapping.target.uniqueKeys ?? []
  const primaryKey = mapping.target.primaryKey

  const checks = [
    ...(primaryKey ? [createRowKeyCheck(primaryKey, { keyType: "primary" })] : []),
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

/** A key whose values are all null is never a duplicate. */
export function createKeyPresenceExpr(keyColumns: string[]) {
  return pl.anyHorizontal(keyColumns.map(name => pl.col(name).isNotNull()))
}
