import { z } from "zod"
import { GithubFile } from "./file.ts"

export const GithubTree = z
  .object({
    tree: z.array(GithubFile).describe("Repository files and directories"),
  })
  .describe("GitHub git tree, as returned by /git/trees/{ref}")

export type GithubTree = z.infer<typeof GithubTree>
