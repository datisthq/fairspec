import type { TableSchema } from "@fairspec/metadata"
import { system } from "../../system.ts"

export async function renderTableSchemaAs(
  tableSchema: TableSchema,
  options: { format: string },
) {
  for (const plugin of system.plugins) {
    const text = plugin.renderTableSchemaAs?.(tableSchema, options)
    if (text) {
      return text
    }
  }

  throw new Error(`No plugin can render the schema as: ${options.format}`)
}
