import { loadDataSchema, validateData } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import { ReportResult } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const validateDataCommand = f
  .command({
    description: "Validate a JSON Data from a local or remote path",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      path: params.requiredFilePath,
      schema: params.dataSchema,
      silent: params.silent,
      debug: params.debug,
    }),
  )
  .output(ReportResult)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })

    const dataSchema = await session.task("Loading data schema", async () => {
      return await loadDataSchema(input.schema)
    })

    const report = await session.task("Validating data", async () => {
      return await validateData({ data: input.path, dataSchema })
    })

    return session.renderReportResult(report)
  })
