import { z } from "zod"
import { CellError } from "./cell.ts"
import { ColumnError } from "./column.ts"
import { ForeignKeyError } from "./foreignKey.ts"
import { RowError } from "./row.ts"

export const TableError = z.union([ColumnError, RowError, CellError, ForeignKeyError])

export type TableError = z.infer<typeof TableError>
