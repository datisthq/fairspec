import { z } from "zod"

export const FrictionlessLicense = z
  .looseObject({
    name: z.string().optional(),
    path: z.string().optional(),
    title: z.string().optional(),
  })
  .describe("Frictionless license")

export type FrictionlessLicense = z.infer<typeof FrictionlessLicense>
