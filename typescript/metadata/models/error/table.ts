import { z } from "zod"
import { CellError } from "./cell.ts"
import { CheckError, TableCheckError } from "./check.ts"
import { ColumnError } from "./column.ts"
import { ForeignKeyError } from "./foreignKey.ts"
import { RowError } from "./row.ts"

export const TableError = z.union([
  ColumnError,
  RowError,
  CellError,
  ForeignKeyError,
  TableCheckError,
  CheckError,
])

export type TableError = z.infer<typeof TableError>
