import type { DataSchema } from "../../models/dataSchema.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import { FairspecException } from "../../models/exception.ts"
import { validateDataSchema } from "./validate.ts"

export async function assertDataSchema(source: Descriptor | DataSchema) {
  const report = await validateDataSchema(source)

  if (!report.dataSchema) {
    throw new FairspecException(`Invalid Data Schema`, { report })
  }

  return report.dataSchema
}
