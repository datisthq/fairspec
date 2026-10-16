import { validateTable } from "@fairspec/library"
import { temporaryDirectoryTask } from "tempy"
import { prefetchFile } from "#actions/prefetch.ts"
import { publicEndpoint } from "#endpoints/base/public.ts"
import { ValidateTableInput } from "#models/table.ts"

export const validateTableEndpoint = publicEndpoint
  .input(ValidateTableInput)
  .handler(async ({ input }) => {
    return await temporaryDirectoryTask(async folder => {
      const [table, schema, dialect] = await Promise.all([
        prefetchFile(input.table, { folder, fileType: "table" }),
        prefetchFile(input.schema, { folder, fileType: "schema" }),
        prefetchFile(input.dialect, { folder, fileType: "dialect" }),
      ])

      const report = await validateTable({
        data: table,
        fileDialect: dialect,
        tableSchema: schema,
      })

      return report
    })
  })
