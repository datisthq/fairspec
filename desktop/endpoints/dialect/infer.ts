import { inferFileDialect } from "@fairspec/library"
import { temporaryDirectoryTask } from "tempy"
import { prefetchFile } from "#actions/prefetch.ts"
import { publicEndpoint } from "#endpoints/base/public.ts"
import { InferDialectInput } from "#models/dialect.ts"

export const inferDialectEndpoint = publicEndpoint
  .input(InferDialectInput)
  .handler(async ({ input }) => {
    return await temporaryDirectoryTask(async folder => {
      const [file] = await Promise.all([
        prefetchFile(input.file, { folder, fileType: "file" }),
      ])

      const dialect = await inferFileDialect({
        data: file,
      })

      return dialect
    })
  })
