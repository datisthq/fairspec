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
  const isErrorExpr = pl
    .concatList(keyColumns)
    .isFirstDistinct()
    .not()
    // Fold is not available so we use a tricky way to eliminate nulls
    .and(pl.concatList(keyColumns).lst.min().isNotNull())

  const errorTemplate: RowPrimaryKeyError | RowUniqueKeyError = {
    type: options.keyType === "primary" ? "row/primaryKey" : "row/uniqueKey",
    columnNames: keyColumns,
    rowNumber: 0,
  }

  return { isErrorExpr, errorTemplate }
}
