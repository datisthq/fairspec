import { z } from "zod"
import { FrictionlessLicense } from "./license.ts"
import { FrictionlessResource, ResolvedFrictionlessResource } from "./resource.ts"

export const FrictionlessContributor = z.looseObject({
  title: z.string().optional(),
  givenName: z.string().optional(),
  familyName: z.string().optional(),
  path: z.string().optional(),
  email: z.string().optional(),
  organization: z.string().optional(),
  role: z.string().optional(),
  roles: z.array(z.string()).optional(),
})

export const FrictionlessSource = z.looseObject({
  title: z.string().optional(),
  path: z.string().optional(),
  email: z.string().optional(),
})

export const FrictionlessPackage = z
  .looseObject({
    $schema: z.string().optional(),
    profile: z.string().optional(),
    name: z.string().optional(),
    id: z.string().optional(),
    title: z.string().optional(),
    description: z.string().optional(),
    homepage: z.string().optional(),
    version: z.string().optional(),
    created: z.string().optional(),
    keywords: z.array(z.string()).optional(),
    licenses: z.array(FrictionlessLicense).optional(),
    contributors: z.array(FrictionlessContributor).optional(),
    sources: z.array(FrictionlessSource).optional(),
    resources: z.array(FrictionlessResource),
  })
  .describe("Frictionless Data Package")

export type FrictionlessContributor = z.infer<typeof FrictionlessContributor>
export type FrictionlessSource = z.infer<typeof FrictionlessSource>
export type FrictionlessPackage = z.infer<typeof FrictionlessPackage>

export const ResolvedFrictionlessPackage = FrictionlessPackage.extend({
  resources: z.array(ResolvedFrictionlessResource),
}).describe("Frictionless Data Package with its resources' schemas and dialects loaded")

export type ResolvedFrictionlessPackage = z.infer<typeof ResolvedFrictionlessPackage>
