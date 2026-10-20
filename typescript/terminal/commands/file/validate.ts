import { validateFile } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import { ReportResult } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const validateFileCommand = f
  .command({
    description: "Validate a file from a local or remote path",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      path: params.requiredFilePath,
      hash: params.hash,
      hashType: params.hashType,
      silent: params.silent,
      debug: params.debug,
    }),
  )
  .output(ReportResult)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })

    const report = await session.task("Validating file", async () => {
      return await validateFile({
        data: input.path,
        integrity: input.hash ? { hash: input.hash, type: input.hashType } : undefined,
      })
    })

    return session.renderReportResult(report)
  })
