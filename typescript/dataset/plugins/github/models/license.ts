import { z } from "zod"

export const GithubLicense = z
  .object({
    key: z.string().describe("License key"),
    name: z.string().describe("License name"),
    spdx_id: z.string().describe("License SPDX ID"),
    url: z.string().describe("License URL"),
  })
  .describe("GitHub repository license")

export type GithubLicense = z.infer<typeof GithubLicense>
