import type { Resource } from "@fairspec/library"
import {
  createReport,
  inferTableSchemaFromTable,
  inspectTable,
  loadTable,
  resolveTableSchema,
} from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { createFileDialectFromPathAndOptions } from "../../helpers/fileDialect.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import { selectResource } from "../../helpers/resource.ts"
import { ReportResult } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const validateTableCommand = f
  .command({
    description: "Validate a table from a local or remote path",
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
      check: params.check,
    }),
  )
  .output(ReportResult)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })
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

    let tableSchema = await session.task("Loading schema", () =>
      resolveTableSchema(resource.tableSchema),
    )

    if (!tableSchema) {
      tableSchema = await session.task("Inferring schema", () =>
        inferTableSchemaFromTable(table, input),
      )
    }

    const checks = input.check?.map(check => {
      const index = check.indexOf("=")
      return { name: check.slice(0, index), condition: check.slice(index + 1) }
    })

    const report = await session.task("Validating table", async () => {
      const errors = await inspectTable(table, { tableSchema, checks })
      return createReport(errors)
    })

    return session.renderReportResult(report)
  })
