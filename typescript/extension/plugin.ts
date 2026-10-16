import type { MetadataPlugin, TableSchema } from "@fairspec/metadata"
import { renderTableSchemaAsHtml } from "./actions/tableSchema/asHtml.tsx"
import { renderTableSchemaAsMarkdown } from "./actions/tableSchema/asMarkdown.ts"

export class ExtensionPlugin implements MetadataPlugin {
  renderTableSchemaAs(tableSchema: TableSchema, options: { format: string }) {
    switch (options.format) {
      case "markdown":
        return renderTableSchemaAsMarkdown(tableSchema)
      case "html":
        return renderTableSchemaAsHtml(tableSchema)
      default:
        return undefined
    }
  }
}
