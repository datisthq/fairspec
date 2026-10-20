import type { Resource } from "@fairspec/library"
import { loadTable, queryTable } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { createFileDialectFromPathAndOptions } from "../../helpers/fileDialect.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import { selectResource } from "../../helpers/resource.ts"
import { Records } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const queryTableCommand = f
  .command({
    description: "Query a table from a local or remote path",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      path: params.tablePath,
      query: params.tableQuery,
      dataset: params.dataset,
      resource: params.resource,
      debug: params.debug,
      ...params.dialectParams,
      schema: params.tableSchema,
      ...params.tableSchemaParams,
    }),
  )
  .output(Records)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })
    const { path, query } = input

    const fileDialect = path
      ? (input.dialect ?? createFileDialectFromPathAndOptions(path, input))
      : undefined

    const resource: Resource = path
      ? { data: path, fileDialect, tableSchema: input.schema }
      : await selectResource(session, input)

    let table = await session.task("Loading table", async () => {
      const table = await loadTable(resource, input)
      if (!table) throw new Error("Could not load table")
      return table
    })

    if (query) {
      table = await session.task("Executing query", async () => {
        return queryTable(table, query)
      })
    }

    const frame = await session.task("Collecting data", async () => {
      return await table.collect()
    })

    return session.renderFrameResult(frame)
  })
