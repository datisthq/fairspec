import type { RowError, TableError, TableSchema } from "@fairspec/metadata"
import { getColumns } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import pAll from "p-all"
import { inspectTableChecks } from "../../actions/check/inspect.ts"
import { inspectColumns } from "../../actions/column/inspect.ts"
import { getPolarsSchema } from "../../helpers/schema.ts"
import type { TableCheck } from "../../models/check.ts"
import type { ColumnMapping } from "../../models/column.ts"
import type { SchemaMapping } from "../../models/schema.ts"
import type { Table } from "../../models/table.ts"
import {
  COUNT_COLUMN_NAME,
  ERROR_COLUMN_NAME,
  INSPECT_ROW_CONCURRENCY,
  NUMBER_COLUMN_NAME,
} from "../../settings.ts"
import { createKeyPresenceExpr, createRowKeyChecks } from "./checks/key.ts"
import { mergeMissingValues } from "./-shared/helpers.ts"

export async function inspectTable(
  table: Table,
  options?: {
    tableSchema?: TableSchema
    checks?: TableCheck[]
    sampleRows?: number
    maxErrors?: number
  },
) {
  const { tableSchema, checks = [], sampleRows = 100, maxErrors = 1000 } = options ?? {}
  const errors: TableError[] = []
  let mapping: SchemaMapping | undefined

  if (tableSchema) {
    const sample = await table.head(sampleRows).collect()
    const polarsSchema = getPolarsSchema(sample.schema)
    mapping = { source: polarsSchema, target: tableSchema }

    const columnErrors = await inspectSchemaColumns(mapping, table, { maxErrors })
    errors.push(...columnErrors)

    const rowErrors = await inspectRows(mapping, table, { maxErrors })
    errors.push(...rowErrors)
  }

  if (checks.length && errors.length < maxErrors) {
    const checkErrors = await inspectTableChecks(table, checks, { mapping, maxErrors })
    errors.push(...checkErrors)
  }

  return errors.slice(0, maxErrors)
}

async function inspectSchemaColumns(
  mapping: SchemaMapping,
  table: Table,
  options: {
    maxErrors: number
  },
) {
  const { maxErrors } = options
  const errors: TableError[] = []
  const columns = getColumns(mapping.target)

  const missingErrors = new Map<string, TableError[]>()
  const columnMappings: ColumnMapping[] = []
  for (const column of columns) {
    const polarsColumn = mapping.source.columns.find(
      polarsColumn => polarsColumn.name === column.name,
    )

    if (!polarsColumn) {
      const isRequired =
        mapping.target.allRequired || mapping.target.required?.includes(column.name)

      missingErrors.set(
        column.name,
        isRequired ? [{ type: "column/missing", columnName: column.name }] : [],
      )
      continue
    }

    const mergedColumn = mergeMissingValues(column, mapping.target)
    columnMappings.push({ source: polarsColumn, target: mergedColumn })
  }

  const cellErrors = await inspectColumns(columnMappings, table, { maxErrors })
  for (const column of columns) {
    errors.push(...(missingErrors.get(column.name) ?? cellErrors.get(column.name) ?? []))
  }

  if (mapping.target.additionalProperties === false) {
    const names = new Set(columns.map(column => column.name))
    for (const polarsColumn of mapping.source.columns) {
      if (names.has(polarsColumn.name)) continue
      errors.push({ type: "column/extra", columnName: polarsColumn.name })
    }
  }

  return errors.sort(
    (a, b) =>
      Number(!a.type.startsWith("column/")) - Number(!b.type.startsWith("column/")),
  )
}

async function inspectRows(
  mapping: SchemaMapping,
  table: Table,
  options: { maxErrors: number },
) {
  const { maxErrors } = options
  const errors: TableError[] = []
  const columns = getColumns(mapping.target)
  const abortController = new AbortController()
  const maxRowErrors = Math.ceil(maxErrors / columns.length)

  const collectRowErrors = async (check: RowKeyCheck) => {
    if (!(await getHasDuplicateKey(table, check))) return

    // A literal is broadcast to every row, so flag the error with a boolean rather
    // than a JSON template, which would materialize a full-length string column.
    // The check owns a single template, so it is read from the closure below.
    const rowCheckTable = table
      .withRowIndex(NUMBER_COLUMN_NAME, 1)
      .withColumn(
        pl
          .when(check.isErrorExpr)
          .then(pl.lit(true))
          .otherwise(pl.lit(null).cast(pl.Bool))
          .alias(ERROR_COLUMN_NAME),
      )

    const rowCheckFrame = await rowCheckTable
      .filter(pl.col(ERROR_COLUMN_NAME).isNotNull())
      .head(maxRowErrors)
      .collect()

    const errorTemplate: RowError = check.errorTemplate
    for (const row of rowCheckFrame.toRecords() as any[]) {
      errors.push({
        ...errorTemplate,
        rowNumber: row[NUMBER_COLUMN_NAME],
      })
    }

    if (errors.length > maxErrors) {
      abortController.abort()
    }
  }

  try {
    await pAll(
      [...createRowKeyChecks(mapping)].map(it => () => collectRowErrors(it)),
      { concurrency: INSPECT_ROW_CONCURRENCY },
    )
  } catch (error) {
    const isAborted = error instanceof Error && error.name === "AbortError"
    if (!isAborted) throw error
  }

  return errors
}

type RowKeyCheck = ReturnType<typeof createRowKeyChecks>[number]

// Grouping reads only the key columns and builds no per-row expression, so a clean key
// costs a fraction of the locating query, which runs only once a duplicate is known
async function getHasDuplicateKey(table: Table, check: RowKeyCheck) {
  const probe = await table
    .select(check.keyColumns)
    .filter(createKeyPresenceExpr(check.keyColumns))
    // maintainOrder: nodejs-polars defaults it to true, an ordered aggregate for no reader
    .groupBy(check.keyColumns, false)
    .len(COUNT_COLUMN_NAME)
    .filter(pl.col(COUNT_COLUMN_NAME).gt(1))
    .head(1)
    .collect()

  return probe.height > 0
}
