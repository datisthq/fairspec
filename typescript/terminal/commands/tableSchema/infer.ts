import type { Resource } from "@fairspec/library"
import { inferTableSchema, TableSchema } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { createFileDialectFromPathAndOptions } from "../../helpers/fileDialect.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import { selectResource } from "../../helpers/resource.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const inferTableSchemaCommand = f
  .command({
    description: "Infer a table schema from a table",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      path: params.tablePath,
      dataset: params.dataset,
      resource: params.resource,
      debug: params.debug,
      ...params.dialectParams,
      ...params.tableSchemaParams,
    }),
  )
  .output(TableSchema)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })
    const path = input.path

    const fileDialect = path
      ? (input.dialect ?? createFileDialectFromPathAndOptions(path, input))
      : undefined

    const resource: Resource = path
      ? { data: path, fileDialect }
      : await selectResource(session, input)

    const tableSchema = await session.task("Inferring schema", async () => {
      const tableSchema = await inferTableSchema(resource, input)

      if (!tableSchema) {
        throw new Error("Could not infer table schema")
      }

      return tableSchema
    })

    return session.renderDataResult(tableSchema)
  })
