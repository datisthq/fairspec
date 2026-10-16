import type { TableSchema } from "@fairspec/metadata"
import { getColumns } from "@fairspec/metadata"
import type { ColumnDef } from "@tanstack/react-table"
import { formatCell } from "./formatters.tsx"

export function createColumnDefs(
  tableSchema: TableSchema,
): ColumnDef<Record<string, any>>[] {
  const columns = getColumns(tableSchema)

  return columns.map(column => {
    const isPrimaryKey = tableSchema.primaryKey?.includes(column.name)

    return {
      accessorKey: column.name,
      header: () => (
        <div className={isPrimaryKey ? "font-bold" : ""}>
          {column.property.title ?? column.name}
          {column.required && <span className="text-red-500 ml-1">*</span>}
        </div>
      ),
      cell: ({ getValue }) => (
        <div className={isPrimaryKey ? "font-medium" : ""}>
          {formatCell(getValue(), column, tableSchema.missingValues)}
        </div>
      ),
    }
  })
}
