import { Kysely } from "kysely"
import { createDialect } from "./createDialect.ts"

export async function connectDatabase(path: string, options?: { create?: boolean }) {
  const dialect = await createDialect(path, options)
  const database = new Kysely<any>({ dialect })
  return database
}
