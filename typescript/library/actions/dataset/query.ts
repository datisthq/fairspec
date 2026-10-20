import type { Dataset } from "@fairspec/metadata"
import { inferResourceName } from "@fairspec/metadata"
import type { LoadTableOptions, Table } from "@fairspec/table"
import { queryTables } from "@fairspec/table"
import { loadTable } from "../../actions/table/load.ts"
import { loadDataset } from "./load.ts"

/**
 * Run a SQL query across a dataset, each table resource exposed under its name.
 * Only the resources the query names are loaded.
 */
export async function queryDataset(
  source: string | Dataset,
  query: string,
  options?: LoadTableOptions,
): Promise<Table> {
  const dataset = typeof source === "string" ? await loadDataset(source) : source
  const words = new Set(query.toLowerCase().match(/[a-z0-9_]+/g))

  const resources = (dataset.resources ?? [])
    .map((resource, index) => ({
      resource,
      name: resource.name ?? inferResourceName(resource, { resourceNumber: index + 1 }),
    }))
    .filter(({ name }) => words.has(name.toLowerCase()))

  const tables: Record<string, Table> = {}
  for (const { resource, name } of resources) {
    if (resources.filter(it => it.name === name).length > 1) {
      throw new Error(`More than one resource is named "${name}"`)
    }

    // A word in the query can be a column or a literal, so a non-table is skipped
    const table = await loadTable(resource, options)
    if (table) tables[name] = table
  }

  return queryTables(tables, query)
}
