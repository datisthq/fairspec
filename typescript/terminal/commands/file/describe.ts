import { describeFile, Integrity } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { selectFile } from "../../helpers/file.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const describeFileCommand = f
  .command({
    description: "Show stats for a local or remote file",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      path: params.filePath,
      dataset: params.dataset,
      resource: params.resource,
      hashType: params.hashType,
      silent: params.silent,
      debug: params.debug,
    }),
  )
  .output(
    z.object({
      bytes: z.number(),
      textual: z.boolean(),
      integrity: Integrity.optional(),
    }),
  )
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })
    const path = input.path ?? (await selectFile(session, input))

    const stats = await session.task("Describing file", async () => {
      return await describeFile(path, { hashType: input.hashType })
    })

    return session.renderDataResult(stats)
  })
