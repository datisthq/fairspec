import { z } from "zod"
import { BaseFileDialect } from "./base.ts"
import { TableName } from "./common.ts"

export const SqliteFileDialect = BaseFileDialect.extend({
  format: z.literal("sqlite"),
  tableName: TableName.optional(),
})

export type SqliteFileDialect = z.infer<typeof SqliteFileDialect>
