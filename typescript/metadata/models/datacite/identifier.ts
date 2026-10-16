import { z } from "zod"

export const Doi = z
  .string()
  .regex(/^10[.][0-9]{4,9}[/][^\s]+$/)
  .describe(
    "The Digital Object Identifier (DOI) is a persistent identifier for the resource, following the DOI syntax",
  )

export const DoiPrefix = z
  .string()
  .regex(/^10[.][0-9]{4,9}$/)
  .describe(
    "The DOI prefix, which is the part of the DOI before the slash. It uniquely identifies the registrant",
  )

export const DoiSuffix = z
  .string()
  .regex(/^[^\s]+$/)
  .describe(
    "The DOI suffix, which is the part of the DOI after the slash. It is assigned by the registrant",
  )

export type Doi = z.infer<typeof Doi>
export type DoiPrefix = z.infer<typeof DoiPrefix>
export type DoiSuffix = z.infer<typeof DoiSuffix>
