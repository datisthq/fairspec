import { Buffer } from "node:buffer"
import { buffer } from "node:stream/consumers"
import type { Dataset, Descriptor } from "@fairspec/metadata"
import { denormalizeDataset, stringifyDescriptor } from "@fairspec/metadata"
import { getDatasetBasepath } from "../../../../actions/dataset/basepath.ts"
import { saveResourceFiles } from "../../../../actions/resource/save.ts"
import { loadFileStream } from "../../../../actions/stream/load.ts"
import type { GithubRepository } from "../../models/repository.ts"
import { makeGithubApiRequest } from "../../services/github.ts"

/**
 * Save a dataset to a Github repository
 * @param options Object containing the package to save and Github details
 * @returns Object with the repository URL
 */
export async function saveDatasetToGithub(
  dataset: Dataset,
  options: {
    apiKey: string
    repo: string
    org?: string
  },
) {
  const { apiKey, org, repo } = options
  const basepath = getDatasetBasepath(dataset)

  const githubRepository = await makeGithubApiRequest<GithubRepository>({
    endpoint: org ? `/orgs/${org}/repos` : "/user/repos",
    payload: { name: repo, auto_init: true },
    method: "POST",
    apiKey,
  })

  const resourceDescriptors: Descriptor[] = []
  for (const resource of dataset.resources ?? []) {
    resourceDescriptors.push(
      await saveResourceFiles(resource, {
        basepath,
        withRemote: false,
        saveFile: async options => {
          const stream = await loadFileStream(options.normalizedPath)

          const payload = {
            path: options.denormalizedPath,
            content: Buffer.from(await buffer(stream)).toString("base64"),
            message: `Added file "${options.denormalizedPath}"`,
          }

          await makeGithubApiRequest({
            endpoint: `/repos/${githubRepository.owner.login}/${repo}/contents/${options.denormalizedPath}`,
            method: "PUT",
            payload,
            apiKey,
          })

          return options.denormalizedPath
        },
      }),
    )
  }

  const descriptor = {
    ...denormalizeDataset(dataset, { basepath }),
    resources: resourceDescriptors,
  }

  for (const denormalizedPath of ["dataset.json"]) {
    const payload = {
      path: denormalizedPath,
      message: `Added file "${denormalizedPath}"`,
      content: Buffer.from(stringifyDescriptor(descriptor)).toString("base64"),
    }

    await makeGithubApiRequest({
      endpoint: `/repos/${githubRepository.owner.login}/${repo}/contents/${denormalizedPath}`,
      method: "PUT",
      payload,
      apiKey,
    })
  }

  return {
    path: `https://raw.githubusercontent.com/${githubRepository.owner.login}/${repo}/refs/heads/main/dataset.json`,
    repoUrl: githubRepository.html_url,
  }
}
