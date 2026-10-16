import type { TableSchema } from "@fairspec/metadata"
import { flexRender, getCoreRowModel, useReactTable } from "@tanstack/react-table"
import { useMemo } from "react"
import {
  Table as BaseTable,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "#elements/table.tsx"
import { cn } from "cn"
import { createColumnDefs } from "./columns.tsx"

interface TableProps {
  records: Record<string, unknown>[]
  tableSchema: TableSchema
  className?: string
}

export function Table({ records, tableSchema, className }: TableProps) {
  const columns = useMemo(() => createColumnDefs(tableSchema), [tableSchema])

  const table = useReactTable({
    data: records,
    columns,
    getCoreRowModel: getCoreRowModel(),
  })

  return (
    <div className={cn("rounded-md border", className)}>
      <BaseTable>
        <TableHeader>
          {table.getHeaderGroups().map(headerGroup => (
            <TableRow key={headerGroup.id}>
              {headerGroup.headers.map(header => (
                <TableHead key={header.id}>
                  {flexRender(header.column.columnDef.header, header.getContext())}
                </TableHead>
              ))}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
          {table.getRowModel().rows.length ? (
            table.getRowModel().rows.map(row => (
              <TableRow key={row.id}>
                {row.getVisibleCells().map(cell => (
                  <TableCell key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableCell>
                ))}
              </TableRow>
            ))
          ) : (
            <TableRow>
              <TableCell colSpan={columns.length} className="h-24 text-center">
                No data available
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </BaseTable>
    </div>
  )
}
