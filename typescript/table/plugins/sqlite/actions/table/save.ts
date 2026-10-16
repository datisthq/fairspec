import { getSupportedFileDialect } from "@fairspec/metadata"
import type { Kysely } from "kysely"
import { denormalizeTable } from "../../../../actions/table/denormalize.ts"
import { inferTableSchemaFromTable } from "../../../../actions/tableSchema/infer.ts"
import type { SaveTableOptions, Table } from "../../../../models/table.ts"
import type { SqliteSchema } from "../../models/schema.ts"
import { NATIVE_TYPES } from "../../settings.ts"
import { connectDatabase } from "../database/connect.ts"
import { convertTableSchemaToDatabase } from "../tableSchema/toDatabase.ts"

// Currently, we use slow non-rust implementation as in the future
// polars-rust might be able to provide a faster native implementation

export async function saveSqliteTable(table: Table, options: SaveTableOptions) {
  const { path, overwrite } = options

  const resource = { data: path, fileDialect: options.fileDialect }
  const fileDialect = await getSupportedFileDialect(resource, ["sqlite"])
  if (!fileDialect) {
    throw new Error("Saving options is not compatible")
  }

  const tableSchema =
    options.tableSchema ??
    (await inferTableSchemaFromTable(table, {
      ...options,
      keepStrings: true,
    }))

  table = await denormalizeTable(table, tableSchema, {
    nativeTypes: NATIVE_TYPES,
  })

  const database = await connectDatabase(path, { create: true })
  try {
    const databaseSchemas = await database.introspection.getTables()

    const tableName =
      fileDialect?.tableName ??
      databaseSchemas.toSorted((a, b) => a.name.localeCompare(b.name))[0]?.name

    if (!tableName) {
      throw new Error("Table name is not defined")
    }

    const sqliteSchema = convertTableSchemaToDatabase(tableSchema, tableName)

    await defineTable(database, sqliteSchema, { overwrite })
    await populateTable(database, tableName, table)

    return path
  } finally {
    await database.destroy()
  }
}

async function defineTable(
  database: Kysely<any>,
  sqliteSchema: SqliteSchema,
  options: {
    overwrite?: boolean
  },
) {
  if (options.overwrite) {
    await database.schema.dropTable(sqliteSchema.name).ifExists().execute()
  }

  let query = database.schema.createTable(sqliteSchema.name)

  for (const field of sqliteSchema.columns) {
    // @ts-expect-error
    query = query.addColumn(field.name, field.dataType)
  }

  if (sqliteSchema.primaryKey) {
    query = query.addPrimaryKeyConstraint(
      `${sqliteSchema.name}_pkey`,
      // @ts-expect-error
      sqliteSchema.primaryKey,
    )
  }

  await query.execute()
}

async function populateTable(database: Kysely<any>, tableName: string, table: Table) {
  let offset = 0
  const frame = await table.collect({ streaming: true })
  while (true) {
    const buffer = frame.slice(offset, offset + BUFFER_SIZE)
    offset += BUFFER_SIZE

    const records = buffer.toRecords()
    if (!records.length) {
      break
    }

    await database.insertInto(tableName).values(records).execute()
  }
}

const BUFFER_SIZE = 10_000
