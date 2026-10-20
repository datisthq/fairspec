import { z } from "zod"

export const GithubContent = z
  .object({
    content: z
      .object({
        path: z.string().describe("File path within repository"),
        sha: z.string().describe("File SHA-1"),
      })
      .describe("Created file"),
  })
  .describe("GitHub file created by /contents/{path}")

export type GithubContent = z.infer<typeof GithubContent>
