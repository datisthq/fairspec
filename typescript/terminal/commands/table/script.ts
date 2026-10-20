import repl from "node:repl"
import type { Resource } from "@fairspec/library"
import * as fairspec from "@fairspec/library"
import { loadTable } from "@fairspec/library"
import { f } from "fireargs"
import pc from "picocolors"
import { z } from "zod"
import { createFileDialectFromPathAndOptions } from "../../helpers/fileDialect.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import { selectResource } from "../../helpers/resource.ts"
import { Empty } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const scriptTableCommand = f
  .command({
    description: "Start a scripting session for a table",
    configureHelp: helpConfiguration,
    jsonOption: false,
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
  .output(Empty)
  .handler(async input => {
    const session = new Session(input)
    const path = input.path

    const fileDialect = path
      ? (input.dialect ?? createFileDialectFromPathAndOptions(path, input))
      : undefined

    const resource: Resource = path
      ? { data: path, fileDialect, tableSchema: input.schema }
      : await selectResource(session, input)

    const table = await session.task("Loading table", async () => {
      const table = await loadTable(resource, { denormalized: true })
      if (!table) throw new Error("Could not load table")
      return table
    })

    session.renderText(
      pc.dim("`fairspec` and `table` variables are available in the session"),
      { status: "warning" },
    )

    const replSession = repl.start({ prompt: "fairspec> " })
    replSession.context.fairspec = fairspec
    replSession.context.table = table

    return {}
  })
