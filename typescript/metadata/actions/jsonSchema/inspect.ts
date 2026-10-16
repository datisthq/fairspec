import type { Descriptor } from "../../models/descriptor.ts"
import { ajv } from "./ajv.ts"

export async function inspectJsonSchema(
  descriptor: Descriptor,
  options?: {
    rootJsonPointer?: string
  },
) {
  const errors: { message: string; jsonPointer: string }[] = []

  await ajv.validateSchema(descriptor)
  for (const error of ajv.errors ?? []) {
    const instancePath = error.instancePath ?? "/"
    const rootPath = options?.rootJsonPointer ?? ""
    const jsonPointer =
      rootPath === "" || rootPath === "/"
        ? instancePath
        : instancePath === "/"
          ? rootPath
          : `${rootPath}${instancePath}`

    errors.push({
      message: error.message ?? error.keyword,
      jsonPointer,
    })
  }

  return errors
}
