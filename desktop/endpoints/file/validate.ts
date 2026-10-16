import { validateFile } from "@fairspec/library"
import { temporaryDirectoryTask } from "tempy"
import { prefetchFile } from "#actions/prefetch.ts"
import { publicEndpoint } from "#endpoints/base/public.ts"
import { ValidateFileInput } from "#models/file.ts"

export const validateFileEndpoint = publicEndpoint
  .input(ValidateFileInput)
  .handler(async ({ input }) => {
    return await temporaryDirectoryTask(async folder => {
      const [file] = await Promise.all([
        prefetchFile(input.file, { folder, fileType: "file" }),
      ])

      const report = await validateFile({
        data: file,
        integrity: {
          type: input.hashType,
          hash: input.hashValue,
        },
      })

      return report
    })
  })
