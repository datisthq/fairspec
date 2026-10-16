import type { Column, TableSchema } from "@fairspec/metadata"
import { getColumns } from "@fairspec/metadata"

export function renderTableSchemaAsMarkdown(
  schema: TableSchema,
  options?: { frontmatter?: boolean },
): string {
  const rows: string[] = []

  rows.push("")
  rows.push("## Columns")
  rows.push("")
  rows.push("| Name | Type | Title | Description | Constraints |")
  rows.push("|------|------|-------|-------------|-------------|")

  const columns = getColumns(schema)
  const required = schema.required ?? []

  for (const column of columns) {
    const { name, type, property } = column
    const title = property.title ?? ""
    const description = property.description ?? ""

    const constraints: string[] = []

    if (required.includes(name)) {
      constraints.push("required")
    }

    constraints.push(...extractConstraints(property))

    const constraintsStr = constraints.join(", ")

    const escapedName = escapeMarkdown(name)
    const escapedTitle = escapeMarkdown(title)
    const escapedDescription = escapeMarkdown(description)
    const escapedConstraints = escapeMarkdown(constraintsStr)

    rows.push(
      `| ${escapedName} | ${type} | ${escapedTitle} | ${escapedDescription} | ${escapedConstraints} |`,
    )
  }

  let markdown = `${rows.join("\n")}\n`

  if (options?.frontmatter) {
    const header: string[] = []
    header.push("---")
    if (schema.title) {
      header.push(`title: ${schema.title}`)
    }
    header.push("---")
    header.push("")
    markdown = header.join("\n") + markdown
  }

  return markdown
}

function escapeMarkdown(text: string): string {
  return text.replace(/\n/g, " ")
}

function extractConstraints(property: Column["property"]): string[] {
  const constraints: string[] = []

  if ("minimum" in property && property.minimum !== undefined) {
    constraints.push(`min: ${property.minimum}`)
  }
  if ("maximum" in property && property.maximum !== undefined) {
    constraints.push(`max: ${property.maximum}`)
  }
  if ("minLength" in property && property.minLength !== undefined) {
    constraints.push(`minLength: ${property.minLength}`)
  }
  if ("maxLength" in property && property.maxLength !== undefined) {
    constraints.push(`maxLength: ${property.maxLength}`)
  }
  if ("pattern" in property && property.pattern) {
    constraints.push(`pattern: ${property.pattern}`)
  }
  if ("enum" in property && property.enum) {
    constraints.push(`enum: ${property.enum.join(", ")}`)
  }

  return constraints
}
