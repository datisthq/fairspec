import type { RowPrimaryKeyError, RowUniqueKeyError } from "@fairspec/metadata"
import { getColumns } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { SchemaMapping } from "../../../models/schema.ts"
import { substituteColumn } from "../../column/substitute.ts"
import { mergeMissingValues } from "../-shared/helpers.ts"

type KeyCheckType = "primary" | "unique"

export function createRowKeyChecks(mapping: SchemaMapping) {
  const names = new Set(mapping.source.columns.map(column => column.name))
  const isPresent = (key: string[]) => key.every(name => names.has(name))
  const uniqueKeys = (mapping.target.uniqueKeys ?? []).filter(isPresent)
  const primaryKey = mapping.target.primaryKey

  const checks = [
    ...(primaryKey && isPresent(primaryKey)
      ? [createRowKeyCheck(mapping, primaryKey, { keyType: "primary" })]
      : []),
    ...uniqueKeys.map(key => createRowKeyCheck(mapping, key, { keyType: "unique" })),
  ]

  return checks
}

/** A key with any null value is never a duplicate. */
export function createKeyPresenceExpr(keyExprs: pl.Expr[]) {
  return pl.allHorizontal(keyExprs.map(expr => expr.isNotNull()))
}

function createRowKeyCheck(
  mapping: SchemaMapping,
  keyColumns: string[],
  options: { keyType: KeyCheckType },
) {
  const keyExprs = keyColumns.map(name => createKeyColumnExpr(mapping, name))
  const isErrorExpr = pl
    .struct(keyExprs)
    .isFirstDistinct()
    .not()
    .and(createKeyPresenceExpr(keyExprs))

  const errorTemplate: RowPrimaryKeyError | RowUniqueKeyError = {
    type: options.keyType === "primary" ? "row/primaryKey" : "row/uniqueKey",
    columnNames: keyColumns,
    rowNumber: 0,
  }

  return { keyColumns, keyExprs, isErrorExpr, errorTemplate }
}

// Keys compare the raw cells, so a missing value must become null here as in the cells
function createKeyColumnExpr(mapping: SchemaMapping, name: string) {
  const source = mapping.source.columns.find(column => column.name === name)
  const column = getColumns(mapping.target).find(column => column.name === name)
  if (!source || !column) return pl.col(name)

  const target = mergeMissingValues(column, mapping.target)
  return substituteColumn({ source, target }, pl.col(name))
}
