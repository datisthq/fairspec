import { z } from "zod"
import { DateType } from "./common.ts"

export const DateValue = z
  .string()
  .describe(
    "Different date formats are supported: YYYY, YYYY-MM, YYYY-MM-DD, YYYY-MM-DDThh:mm:ss, YYYY-MM-DDThh:mm:ssTZD, or any of these formats with ranges separated by /",
  )

export const Date = z.object({
  date: DateValue.describe(
    "The date associated with an event in the lifecycle of the resource",
  ),
  dateType: DateType.describe(
    "The type of date (e.g., Accepted, Available, Created, Issued, Submitted, Updated, etc.)",
  ),
  dateInformation: z
    .string()
    .optional()
    .describe("Additional information about the date"),
})

export const Dates = z.array(Date).describe("Different dates relevant to the work")

export type DateValue = z.infer<typeof DateValue>
export type Date = z.infer<typeof Date>
export type Dates = z.infer<typeof Dates>
