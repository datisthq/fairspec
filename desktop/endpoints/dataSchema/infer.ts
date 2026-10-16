import { inferDataSchema } from "@fairspec/library"
import { temporaryDirectoryTask } from "tempy"
import { prefetchFile } from "#actions/prefetch.ts"
import { publicEndpoint } from "#endpoints/base/public.ts"
import { InferDataSchemaInput } from "#models/dataSchema.ts"

export const inferDataSchemaEndpoint = publicEndpoint
  .input(InferDataSchemaInput)
  .handler(async ({ input }) => {
    return await temporaryDirectoryTask(async folder => {
      const [data] = await Promise.all([
        prefetchFile(input.data, { folder, fileType: "data" }),
      ])

      const schema = await inferDataSchema({
        data,
      })

      return schema
    })
  })
