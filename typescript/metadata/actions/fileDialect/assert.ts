import type { Descriptor } from "../../models/descriptor.ts"
import { FairspecException } from "../../models/exception.ts"
import type { FileDialect } from "../../models/fileDialect/fileDialect.ts"
import { validateFileDialect } from "./validate.ts"

export async function assertFileDialect(source: Descriptor | FileDialect) {
  const report = await validateFileDialect(source)

  if (!report.fileDialect) {
    throw new FairspecException(`Invalid dialect`, { report })
  }

  return report.fileDialect
}
