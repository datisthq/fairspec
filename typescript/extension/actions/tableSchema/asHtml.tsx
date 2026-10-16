import type { Column, TableSchema } from "@fairspec/metadata"
import { getColumns } from "@fairspec/metadata"
import { prettify } from "htmlfy"
import { renderToStaticMarkup } from "react-dom/server"

export function renderTableSchemaAsHtml(
  schema: TableSchema,
  options?: { frontmatter?: boolean },
): string {
  let html = prettify(renderToStaticMarkup(<SchemaTable schema={schema} />))

  if (options?.frontmatter) {
    if (schema.title) {
      html = `---\ntitle: ${schema.title}\n---\n\n${html}`
    }
  }

  return html
}

function SchemaTable(props: { schema: TableSchema }) {
  const { schema } = props

  return (
    <>
      {schema.primaryKey && <PrimaryKey fields={schema.primaryKey} />}
      {schema.foreignKeys && schema.foreignKeys.length > 0 && (
        <ForeignKeys foreignKeys={schema.foreignKeys} />
      )}
      <FieldsTable schema={schema} />
    </>
  )
}

function FieldsTable(props: { schema: TableSchema }) {
  const { schema } = props
  const columns = getColumns(schema)
  const required = schema.required ?? []

  return (
    <>
      <h2>Columns</h2>
      <table>
        <colgroup>
          <col width="20%" />
          <col width="65%" />
          <col width="15%" />
        </colgroup>
        <thead>
          <tr>
            <th>Name</th>
            <th>Definition</th>
            <th>Type</th>
          </tr>
        </thead>
        <tbody>
          {columns.map(column => (
            <FieldRow
              key={column.name}
              column={column}
              isRequired={required.includes(column.name)}
            />
          ))}
        </tbody>
      </table>
    </>
  )
}

function FieldRow(props: { column: Column; isRequired: boolean }) {
  const { column, isRequired } = props
  const { name, type, property } = column
  const columnDescription = property.description ?? ""

  const constraints = extractConstraints(property)

  return (
    <tr>
      <td id={sanitizeId(name)}>
        <code>
          <strong>
            {name}
            {!isRequired && "?"}
          </strong>
        </code>
      </td>
      <td>
        {columnDescription && <p>{columnDescription}</p>}
        {constraints.length > 0 && <ConstraintsList constraints={constraints} />}
        {"categories" in property && property.categories !== undefined && (
          <CategoriesList categories={property.categories} />
        )}
      </td>
      <td>
        <code>{type}</code>
      </td>
    </tr>
  )
}

function ConstraintsList(props: { constraints: Constraint[] }) {
  const { constraints } = props
  return (
    <>
      <strong>Constraints</strong>
      <ul>
        {constraints.map((constraint, index) => (
          <li key={index}>
            {constraint.name}: <code>{constraint.value}</code>
          </li>
        ))}
      </ul>
    </>
  )
}

function CategoriesList(props: { categories: any[] }) {
  const { categories } = props
  return (
    <>
      <strong>Categories</strong>
      <ul>
        {categories.map((category, index) => {
          const value = typeof category === "object" ? category.value : category
          const label = typeof category === "object" ? category.label : undefined
          return (
            <li key={index}>
              <code>{String(value)}</code>
              {label && ` - ${label}`}
            </li>
          )
        })}
      </ul>
    </>
  )
}

function extractConstraints(property: Column["property"]): Constraint[] {
  const constraints: Constraint[] = []

  if ("minimum" in property && property.minimum !== undefined) {
    constraints.push({ name: "minimum", value: String(property.minimum) })
  }
  if ("maximum" in property && property.maximum !== undefined) {
    constraints.push({ name: "maximum", value: String(property.maximum) })
  }
  if ("minLength" in property && property.minLength !== undefined) {
    constraints.push({ name: "minLength", value: String(property.minLength) })
  }
  if ("maxLength" in property && property.maxLength !== undefined) {
    constraints.push({ name: "maxLength", value: String(property.maxLength) })
  }
  if ("pattern" in property && property.pattern) {
    constraints.push({ name: "pattern", value: property.pattern })
  }
  if ("enum" in property && property.enum) {
    const enumValues = property.enum.map(v => String(v)).join(", ")
    constraints.push({ name: "enum", value: enumValues })
  }

  return constraints
}

function PrimaryKey(props: { fields: string[] }) {
  const { fields } = props
  return (
    <>
      <h2>Primary Key</h2>
      <p>
        <code>{fields.join(", ")}</code>
      </p>
    </>
  )
}

function ForeignKeys(props: { foreignKeys: TableSchema["foreignKeys"] }) {
  const { foreignKeys } = props
  if (!foreignKeys) return null

  return (
    <>
      <h2>Foreign Keys</h2>
      <table>
        <colgroup>
          <col width="40%" />
          <col width="30%" />
          <col width="30%" />
        </colgroup>
        <thead>
          <tr>
            <th>Columns</th>
            <th>Reference Resource</th>
            <th>Reference Columns</th>
          </tr>
        </thead>
        <tbody>
          {foreignKeys.map((fk, index) => (
            <tr key={index}>
              <td>
                <code>{fk.columns.join(", ")}</code>
              </td>
              <td>
                <code>{fk.reference.resource || "-"}</code>
              </td>
              <td>
                <code>{fk.reference.columns.join(", ")}</code>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  )
}

function sanitizeId(text: string): string {
  return text
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "")
}

interface Constraint {
  name: string
  value: string
}
