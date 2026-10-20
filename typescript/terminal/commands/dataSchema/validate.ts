import { validateDataSchema } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import { ReportResult } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const validateDataSchemaCommand = f
  .command({ description: "Validate a Data Schema", configureHelp: helpConfiguration })
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

    const report = await session.task("Validating data", async () => {
      const { valid, errors } = await validateDataSchema(input.path)
      return { valid, errors }
    })

    return session.renderReportResult(report)
  })
