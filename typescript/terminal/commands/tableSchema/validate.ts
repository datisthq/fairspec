import { validateTableSchema } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import { ReportResult } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const validateTableSchemaCommand = f
  .command({ description: "Validate a Table Schema", configureHelp: helpConfiguration })
  .input(
    z.object({
      path: params.requiredFilePath,
      silent: params.silent,
      debug: params.debug,
    }),
  )
  .output(ReportResult)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })

    const report = await session.task("Validating table schema", async () => {
      const { valid, errors } = await validateTableSchema(input.path)
      return { valid, errors }
    })

    return session.renderReportResult(report)
  })
