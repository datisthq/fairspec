import type { Resource } from "@fairspec/metadata"
import { getDataFirstPath, getSupportedFileDialect } from "@fairspec/metadata"
import { connectDatabase } from "../database/connect.ts"
import { convertTableSchemaFromDatabase } from "./fromDatabase.ts"

export async function inferTableSchemaFromSqlite(resource: Resource) {
  const firstPath = getDataFirstPath(resource)
  if (!firstPath) {
    throw new Error("Database is not defined")
  }

  const dialect = await getSupportedFileDialect(resource, ["sqlite"])
  if (!dialect) {
    throw new Error("Resource data is not compatible")
  }

  const database = await connectDatabase(firstPath)
  try {
    const databaseSchemas = await database.introspection.getTables()

    const tableName =
      dialect?.tableName ??
      databaseSchemas.toSorted((a, b) => a.name.localeCompare(b.name))[0]?.name

    if (!tableName) {
      throw new Error("Table name is not defined")
    }

    const databaseSchema = databaseSchemas.find(s => s.name === tableName)
    if (!databaseSchema) {
      throw new Error(`Table is not found in the database: ${tableName}`)
    }

    return convertTableSchemaFromDatabase(databaseSchema)
  } finally {
    await database.destroy()
  }
}
