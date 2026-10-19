import { resolveTableSchema } from "@fairspec/metadata"
import { getSupportedFileDialect } from "@fairspec/metadata"
import { getDataFirstPath } from "@fairspec/metadata"
import type { Resource } from "@fairspec/metadata"
import { normalizeTable } from "../../../../actions/table/normalize.ts"
import type { LoadTableOptions } from "../../../../models/table.ts"
import * as pl from "nodejs-polars"
import { connectDatabase } from "../database/connect.ts"
import type { SqliteSchema } from "../../models/schema.ts"
import { inferTableSchemaFromSqlite } from "../tableSchema/infer.ts"

// Currently, we use slow non-rust implementation as in the future
// polars-rust might be able to provide a faster native implementation

export async function loadSqliteTable(resource: Resource, options?: LoadTableOptions) {
  const firstPath = getDataFirstPath(resource)
  if (!firstPath) {
    throw new Error("Resource path is not defined")
  }

  let fileDialect = await getSupportedFileDialect(resource, ["sqlite"])
  if (!fileDialect) {
    throw new Error("Resource data is not compatible")
  }

  const database = await connectDatabase(firstPath)
  try {
    const databaseSchemas = await database.introspection.getTables()

    const tableName =
      fileDialect?.tableName ??
      databaseSchemas.toSorted((a, b) => a.name.localeCompare(b.name))[0]?.name

    if (!tableName) {
      throw new Error("Table name is not defined")
    }

    const records = await database.selectFrom(tableName).selectAll().execute()
    const databaseSchema = databaseSchemas.find(schema => schema.name === tableName)
    let frame = pl.DataFrame(records)
    if (databaseSchema) {
      frame = castIntegerColumns(frame, databaseSchema)
    }

    let table = frame.lazy()
    if (!options?.denormalized) {
      let tableSchema = await resolveTableSchema(resource.tableSchema)
      if (!tableSchema) {
        tableSchema = await inferTableSchemaFromSqlite({
          ...resource,
          fileDialect,
        })
      }

      table = await normalizeTable(table, tableSchema)
    }

    return table
  } finally {
    await database.destroy()
  }
}

// JS numbers all load as Float64, so integer columns take back their declared type;
// SQLite lets any value into an INTEGER column, so only all-integer ones are cast
function castIntegerColumns(frame: pl.DataFrame, databaseSchema: SqliteSchema) {
  const names = databaseSchema.columns
    .filter(column => column.dataType.toLowerCase() === "integer")
    .map(column => column.name)
    .filter(name => frame.columns.includes(name))
    .filter(name => {
      const series = frame.getColumn(name)
      if (!series.dtype.equals(pl.Float64)) return false
      return series.toArray().every(value => value === null || Number.isInteger(value))
    })

  return names.length
    ? frame.withColumns(...names.map(name => pl.col(name).cast(pl.Int64)))
    : frame
}
