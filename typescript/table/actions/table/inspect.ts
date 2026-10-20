import type { RowError, TableError, TableSchema } from "@fairspec/metadata"
import { getColumns } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import pAll from "p-all"
import { inspectColumns } from "../../actions/column/inspect.ts"
import { getPolarsSchema } from "../../helpers/schema.ts"
import type { ColumnMapping } from "../../models/column.ts"
import type { SchemaMapping } from "../../models/schema.ts"
import type { Table } from "../../models/table.ts"
import {
  ERROR_COLUMN_NAME,
  INSPECT_ROW_CONCURRENCY,
  NUMBER_COLUMN_NAME,
} from "../../settings.ts"
import { createRowKeyChecks } from "./checks/key.ts"
import { mergeMissingValues } from "./-shared/helpers.ts"

export async function inspectTable(
  table: Table,
  options?: {
    tableSchema?: TableSchema
    sampleRows?: number
    maxErrors?: number
  },
) {
  const { tableSchema, sampleRows = 100, maxErrors = 1000 } = options ?? {}
  const errors: TableError[] = []

  if (tableSchema) {
    const sample = await table.head(sampleRows).collect()
    const polarsSchema = getPolarsSchema(sample.schema)
    const mapping = { source: polarsSchema, target: tableSchema }

    const columnErrors = await inspectSchemaColumns(mapping, table, { maxErrors })
    errors.push(...columnErrors)

    const rowErrors = await inspectRows(mapping, table, { maxErrors })
    errors.push(...rowErrors)
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

  const collectRowErrors = async (check: any) => {
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

    const errorTemplate = check.errorTemplate as RowError
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
