import type { FileError, MetadataError, Resource } from "@fairspec/metadata"
import { createReport } from "@fairspec/metadata"
import { inferHash, inferTextual } from "./infer.ts"

const HASH_LENGTHS = { md5: 32, sha1: 40, sha256: 64, sha512: 128 }

export async function validateFile(resource: Resource) {
  const errors: (FileError | MetadataError)[] = []

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
    const length = HASH_LENGTHS[resource.integrity.type]

    if (!new RegExp(`^[0-9a-f]{${length}}$`).test(expectedHash)) {
      errors.push({
        type: "metadata",
        message: `must be ${length} lowercase hexadecimal digits`,
        jsonPointer: "/integrity/hash",
      })
    } else {
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
  }

  return createReport(errors)
}
