import { z } from "zod"
import { FrictionlessDialect } from "./dialect.ts"
import { FrictionlessLicense } from "./license.ts"
import { FrictionlessSchema } from "./schema.ts"

export const FrictionlessResource = z
  .looseObject({
    name: z.string().optional(),
    path: z.union([z.string(), z.array(z.string())]).optional(),
    data: z.unknown().optional(),
    title: z.string().optional(),
    description: z.string().optional(),
    format: z.string().optional(),
    mediatype: z.string().optional(),
    encoding: z.string().optional(),
    bytes: z.int().optional(),
    hash: z.string().optional(),
    licenses: z.array(FrictionlessLicense).optional(),
    dialect: z.union([z.string(), FrictionlessDialect]).optional(),
    schema: z.union([z.string(), FrictionlessSchema]).optional(),
  })
  .describe("Frictionless Data Resource")

export type FrictionlessResource = z.infer<typeof FrictionlessResource>
