import type { Descriptor } from "../../models/descriptor.ts"
import { FairspecException } from "../../models/exception.ts"
import type { TableSchema } from "../../models/tableSchema.ts"
import { validateTableSchema } from "./validate.ts"

export async function assertTableSchema(source: Descriptor | TableSchema) {
  const report = await validateTableSchema(source)

  if (!report.tableSchema) {
    throw new FairspecException(`Invalid Table Schema`, { report })
  }

  return report.tableSchema
}
