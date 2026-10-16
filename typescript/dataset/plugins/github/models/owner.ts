import { z } from "zod"

export const GithubOwner = z
  .object({
    login: z.string().describe("Owner login name"),
    id: z.number().describe("Owner ID"),
    avatar_url: z.string().describe("Owner avatar URL"),
    html_url: z.string().describe("Owner URL"),
    type: z.enum(["User", "Organization"]).describe("Owner type (User/Organization)"),
  })
  .describe("GitHub repository owner")

export type GithubOwner = z.infer<typeof GithubOwner>
