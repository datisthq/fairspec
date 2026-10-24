import type { Dataset, FairspecError } from "@fairspec/metadata"
import { createReport, inferResourceName, resolveTableSchema } from "@fairspec/metadata"
import type { Table } from "@fairspec/table"
import { loadTable } from "../../actions/table/load.ts"

// TODO: review temporary files creation from validateDataset call

export async function validateDatasetForeignKeys(
  dataset: Dataset,
  options?: { maxErrors?: number },
) {
  const { maxErrors = 1000 } = options ?? {}

  const errors: FairspecError[] = []
  const tables: Record<string, Table> = {}

  const resources = (dataset.resources ?? []).map((resource, index) => ({
    resource,
    name: resource.name ?? inferResourceName(resource, { resourceNumber: index + 1 }),
  }))

  for (const { resource, name: resourceName } of resources) {
    const tableSchema = await resolveTableSchema(resource.tableSchema)
    if (!tableSchema) continue

    const foreignKeys = tableSchema.foreignKeys
    if (!foreignKeys) continue

    const names = [
      resourceName,
      ...foreignKeys.flatMap(it =>
        it.reference.resource ? [it.reference.resource] : [],
      ),
    ]

    for (const name of names) {
      const resource = resources.find(it => it.name === name)?.resource

      if (!resource) {
        errors.push({
          type: "resource/missing",
          resourceName: name,
          referencingResourceName: resourceName,
        })

        continue
      }

      if (!tables[name]) {
        const table = await loadTable(resource)

        if (!table) {
          errors.push({
            type: "resource/type",
            resourceName: name,
            expectedResourceType: "table",
            referencingResourceName: resourceName,
          })

          continue
        }

        tables[name] = table
      }
    }

    for (const foreignKey of foreignKeys) {
      const left = tables[resourceName]
      const right = tables[foreignKey.reference.resource ?? resourceName]

      // Missing resource error added in the previous step
      if (!left || !right) {
        continue
      }

      const foreignKeyCheckTable = left
        .select(...foreignKey.columns)
        .dropNulls(foreignKey.columns)
        .join(right, {
          how: "anti",
          leftOn: foreignKey.columns,
          rightOn: foreignKey.reference.columns,
        })

      const foreignKeyCheckFrame = await foreignKeyCheckTable.head(maxErrors).collect()

      for (const row of foreignKeyCheckFrame.toRecords() as any[]) {
        errors.push({
          type: "foreignKey",
          foreignKey,
          cells: Object.values(row).map(String),
          resourceName,
        })
      }
    }
  }

  return createReport(errors, { maxErrors })
}
