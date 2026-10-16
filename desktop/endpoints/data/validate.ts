import { validateData } from "@fairspec/library"
import { temporaryDirectoryTask } from "tempy"
import { prefetchFile } from "#actions/prefetch.ts"
import { publicEndpoint } from "#endpoints/base/public.ts"
import { ValidateDataInput } from "#models/data.ts"

export const validateDataEndpoint = publicEndpoint
  .input(ValidateDataInput)
  .handler(async ({ input }) => {
    return await temporaryDirectoryTask(async folder => {
      const [data, schema] = await Promise.all([
        prefetchFile(input.data, { folder, fileType: "data" }),
        prefetchFile(input.schema, { folder, fileType: "schema" }),
      ])

      const report = await validateData({
        data: data,
        dataSchema: schema,
      })

      return report
    })
  })
