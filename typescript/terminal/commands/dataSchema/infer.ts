import { DataSchema, inferDataSchema } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const inferDataSchemaCommand = f
  .command({ description: "Infer a Data Schema", configureHelp: helpConfiguration })
  .input(
    z.object({
      path: params.requiredFilePath,
      silent: params.silent,
      debug: params.debug,
    }),
  )
  .output(DataSchema)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })

    const dataSchema = await session.task("Infering data schema", async () => {
      const dataSchema = await inferDataSchema({ data: input.path })

      if (!dataSchema) {
        throw new Error("Could not infer data schema")
      }

      return dataSchema
    })

    return session.renderDataResult(dataSchema)
  })
