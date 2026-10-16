import type { Resource } from "@fairspec/metadata"
import type { LoadTableOptions } from "@fairspec/table"
import { system } from "../../system.ts"

export async function loadTable(resource: Resource, options?: LoadTableOptions) {
  for (const plugin of system.plugins) {
    const table = await plugin.loadTable?.(resource, options)
    if (table) {
      return table
    }
  }

  return undefined
}
