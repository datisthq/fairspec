import { z } from "zod"
import { BaseError } from "./base.ts"

export const TableCheckError = BaseError.extend({
  type: z.literal("table/check").describe("Error type identifier"),
  checkName: z.string().describe("The name of the check that failed"),
  message: z.string().optional().describe("The message of the check that failed"),
})

export const CheckError = BaseError.extend({
  type: z.literal("check").describe("Error type identifier"),
  checkName: z.string().describe("The name of the check that cannot be evaluated"),
  message: z.string().describe("Why the check cannot be evaluated"),
})

export type TableCheckError = z.infer<typeof TableCheckError>
export type CheckError = z.infer<typeof CheckError>
