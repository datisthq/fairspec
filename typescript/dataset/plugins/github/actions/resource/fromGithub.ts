import type { Resource } from "@fairspec/metadata"
import { getFileNameSlug } from "@fairspec/metadata"
import type { GithubFile } from "../../models/file.ts"

export function convertResourceFromGithub(
  githubFile: GithubFile,
  options: {
    defaultBranch: string
  },
) {
  const path = convertPath({
    ...githubFile,
    ref: options.defaultBranch,
  })

  const resource: Resource = {
    data: path,
    name: getFileNameSlug(path) ?? githubFile.sha,
    integrity: {
      type: "sha1",
      hash: githubFile.sha,
    },
    unstable_customMetadata: {
      githubKey: githubFile.path,
      githubUrl: path,
    },
  }

  return resource
}

function convertPath(options: { url: string; ref: string; path: string }) {
  const url = new URL(options.url)
  const [owner, repo] = url.pathname.split("/").slice(2)
  return `https://raw.githubusercontent.com/${owner}/${repo}/refs/heads/${options.ref}/${options.path}`
}
