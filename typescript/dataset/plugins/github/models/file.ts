import { z } from "zod"

export const GithubFile = z
  .object({
    path: z.string().describe("File path within repository"),
    mode: z.string().describe("File mode e.g. `100755`"),
    type: z.string().describe("File type e.g. `blob`"),
    size: z.number().describe("File size in bytes"),
    sha: z.string().describe("File SHA-1"),
    url: z.string().describe("File url on GitHub API"),
  })
  .describe("GitHub repository file content")

export type GithubFile = z.infer<typeof GithubFile>
