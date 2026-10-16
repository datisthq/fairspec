import type { FileError, Resource } from "@fairspec/metadata"
import { createReport } from "@fairspec/metadata"
import { inferHash, inferTextual } from "./infer.ts"

export async function validateFile(resource: Resource) {
  const errors: FileError[] = []

  if (resource.textual) {
    const actualTextual = await inferTextual(resource)

    if (!actualTextual) {
      errors.push({
        type: "file/textual",
      })
    }
  }

  if (resource.integrity) {
    const expectedHash = resource.integrity.hash
    const actualHash = await inferHash(resource, {
      hashType: resource.integrity.type,
    })

    if (actualHash !== expectedHash) {
      errors.push({
        type: "file/integrity",
        hashType: resource.integrity.type,
        expectedHash,
        actualHash: actualHash ?? "",
      })
    }
  }

  return createReport(errors)
}
