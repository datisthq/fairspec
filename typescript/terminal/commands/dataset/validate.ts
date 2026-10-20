import { validateDataset } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import { ReportResult } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const validateDatasetCommand = f
  .command({
    description: "Validate a dataset from a local or remote path",
    configureHelp: helpConfiguration,
  })
  .input(z.object({ path: params.descriptorPath, debug: params.debug }))
  .output(ReportResult)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })

    const report = await session.task("Validating dataset", async () => {
      return await validateDataset(input.path)
    })

    return session.renderReportResult(report)
  })
