import type { RowPrimaryKeyError, RowUniqueKeyError } from "@fairspec/metadata"
import { getColumns } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { SchemaMapping } from "../../../models/schema.ts"
import { normalizeColumn } from "../../column/normalize.ts"
import { mergeMissingValues } from "../-shared/helpers.ts"

type KeyCheckType = "primary" | "unique"

export function createRowKeyChecks(
  mapping: SchemaMapping,
  options?: { plainIntegerColumns?: ReadonlySet<string> },
) {
  const plainIntegerColumns = options?.plainIntegerColumns ?? new Set<string>()
  const names = new Set(mapping.source.columns.map(column => column.name))
  const isPresent = (key: string[]) => key.every(name => names.has(name))
  const uniqueKeys = (mapping.target.uniqueKeys ?? []).filter(isPresent)
  const primaryKey = mapping.target.primaryKey

  const checks = [
    ...(primaryKey && isPresent(primaryKey)
      ? [
          createRowKeyCheck(mapping, primaryKey, {
            keyType: "primary",
            plainIntegerColumns,
          }),
        ]
      : []),
    ...uniqueKeys.map(key =>
      createRowKeyCheck(mapping, key, { keyType: "unique", plainIntegerColumns }),
    ),
  ]

  return checks
}

export type RowKeyCheck = ReturnType<typeof createRowKeyChecks>[number]

/** Records the hash of every present key while another query reads the table. */
export function createKeyHashProbe(check: RowKeyCheck) {
  const hashes = pl.Series("", [], pl.UInt64)
  const falses = new Map<number, pl.Series>()

  // Called per batch. It returns all-false, so the flag joins the reading query's
  // filter without changing its rows
  const recordHashes = (batch: pl.Series) => {
    hashes.append(batch)
    let result = falses.get(batch.length)
    if (!result) {
      result = pl.Series("", new Array(batch.length).fill(false), pl.Bool)
      falses.set(batch.length, result)
    }
    return result
  }

  // A key column the reading query already parses is read from it: the streaming
  // engine does not share a repeated expression, so parsing it again costs a second read
  const createFlagExpr = (readTyped: (name: string) => pl.Expr | undefined) => {
    const keys = check.keyColumns.flatMap((name, index) => {
      const key = readTyped(name) ?? check.keyExprs[index]
      return key ? [key] : []
    })
    const [firstKey] = keys
    const keyExpr = keys.length === 1 && firstKey ? firstKey : pl.struct(keys)
    // nodejs-polars plans `hash` as its input's dtype, so without the cast the null
    // branch widens it to Float64, and a struct key fails to plan
    return pl
      .when(createKeyPresenceExpr(keys))
      .then(keyExpr.hash().cast(pl.UInt64))
      .otherwise(pl.lit(null).cast(pl.UInt64))
      .mapBatches(recordHashes, { returnType: pl.Bool, isElementwise: true })
  }

  const getHasDuplicateKey = () => {
    const presentHashes = hashes.dropNulls()
    return presentHashes.nUnique() < presentHashes.length
  }

  return { createFlagExpr, getHasDuplicateKey }
}

/** A key with any null value is never a duplicate. */
export function createKeyPresenceExpr(keyExprs: pl.Expr[]) {
  return pl.allHorizontal(keyExprs.map(expr => expr.isNotNull()))
}

function createRowKeyCheck(
  mapping: SchemaMapping,
  keyColumns: string[],
  options: { keyType: KeyCheckType; plainIntegerColumns: ReadonlySet<string> },
) {
  const keyExprs = keyColumns.map(name =>
    createKeyColumnExpr(mapping, name, options.plainIntegerColumns.has(name)),
  )
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

// Keys compare the typed values, so `1.0` and `1` in a number column are one key
function createKeyColumnExpr(mapping: SchemaMapping, name: string, isPlain: boolean) {
  const source = mapping.source.columns.find(column => column.name === name)
  const column = getColumns(mapping.target).find(column => column.name === name)
  if (!source || !column) return pl.col(name)

  const target = mergeMissingValues(column, mapping.target)
  return normalizeColumn({ source, target }, { isPlain })
}
