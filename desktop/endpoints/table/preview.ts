import type { Resource } from "@fairspec/library"
import {
  inferTableSchemaFromTable,
  loadTable,
  resolveTableSchema,
} from "@fairspec/library"
import { temporaryDirectoryTask } from "tempy"
import { prefetchFile } from "#actions/prefetch.ts"
import { publicEndpoint } from "#endpoints/base/public.ts"
import { PreviewTableInput } from "#models/table.ts"

export const previewTableEndpoint = publicEndpoint
  .input(PreviewTableInput)
  .handler(async ({ input }) => {
    return await temporaryDirectoryTask(async folder => {
      const [data, schema, dialect] = await Promise.all([
        prefetchFile(input.table, { folder, fileType: "table" }),
        prefetchFile(input.schema, { folder, fileType: "schema" }),
        prefetchFile(input.dialect, { folder, fileType: "dialect" }),
      ])

      const resource: Resource = {
        data,
        fileDialect: dialect,
        tableSchema: schema,
      }

      const table = await loadTable(resource, { previewBytes: 10_000 })
      if (!table) {
        throw new Error("Failed to load table")
      }

      let tableSchema = await resolveTableSchema(resource.tableSchema)
      if (!tableSchema) {
        tableSchema = await inferTableSchemaFromTable(table)
      }

      const frame = await table.limit(100).collect()
      const records = frame.toRecords()

      return { records, tableSchema }
    })
  })
