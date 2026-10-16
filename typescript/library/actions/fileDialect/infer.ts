import type { InferFileDialectOptions } from "@fairspec/dataset"
import type { Resource } from "@fairspec/metadata"
import { system } from "../../system.ts"

// TODO: review default values being {} vs undefined

export async function inferFileDialect(
  resource: Resource,
  options?: InferFileDialectOptions,
) {
  for (const plugin of system.plugins) {
    const fileDialect = await plugin.inferFileDialect?.(resource, options)
    if (fileDialect) {
      return fileDialect
    }
  }

  return undefined
}
