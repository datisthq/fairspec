import { DatabaseSync } from "node:sqlite"
import { getIsLocalPathExist } from "@fairspec/dataset"
import { GenericSqliteDialect } from "kysely-generic-sqlite"
import { createSqliteExecutor } from "./createExecutor.ts"

export async function createDialect(path: string, options?: { create?: boolean }) {
  path = path.replace(/^sqlite:\/\//, "")

  if (path === ":memory:" || path.startsWith("file::memory")) {
    throw new Error("In-memory databases are not supported")
  }

  if (!options?.create) {
    const isExist = await getIsLocalPathExist(path)
    if (!isExist) {
      throw new Error(`Database file "${path}" does not exist`)
    }
  }

  // TODO: Currently, the solution is not optimal / hacky
  // We need to rebase on proper sqlite dialect when it will be available
  // - https://github.com/kysely-org/kysely/issues/1292
  // - https://github.com/oven-sh/bun/issues/20412
  return new GenericSqliteDialect(() => createSqliteExecutor(new DatabaseSync(path)))
}
