import type { Resource } from "@fairspec/library"
import { loadTable } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { createFileDialectFromPathAndOptions } from "../../helpers/fileDialect.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import { selectResource } from "../../helpers/resource.ts"
import { Records } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const previewTableCommand = f
  .command({
    description: "Preview a table from a local or remote path",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      path: params.tablePath,
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
    const path = input.path

    const fileDialect = path
      ? (input.dialect ?? createFileDialectFromPathAndOptions(path, input))
      : undefined

    const resource: Resource = path
      ? { data: path, fileDialect, tableSchema: input.schema }
      : await selectResource(session, input)

    const table = await session.task("Loading table preview", async () => {
      const table = await loadTable(resource, { ...input, previewBytes: 10_000 })
      if (!table) throw new Error("Could not load table")
      return table
    })

    const frame = await session.task("Collecting data", async () => {
      return await table.limit(100).collect()
    })

    return session.renderFrameResult(frame)
  })
