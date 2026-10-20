import { FileDialect, inferFileDialect } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { selectFile } from "../../helpers/file.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const inferFileDialectCommand = f
  .command({
    description: "Infer the dialect of a file",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      path: params.filePath,
      dataset: params.dataset,
      resource: params.resource,
      sampleBytes: params.sampleBytes,
      silent: params.silent,
      debug: params.debug,
    }),
  )
  .output(FileDialect)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })
    const path = input.path ?? (await selectFile(session, input))

    const fileDialect = await session.task("Inferring dialect", async () => {
      const fileDialect = await inferFileDialect(
        { data: path },
        { sampleBytes: input.sampleBytes },
      )

      if (!fileDialect) {
        throw new Error("Could not infer dialect")
      }

      return fileDialect
    })

    return session.renderDataResult(fileDialect)
  })
