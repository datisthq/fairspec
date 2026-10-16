import type { Dataset } from "@fairspec/metadata"
import { convertResourceFromGithub } from "../../actions/resource/fromGithub.ts"
import type { GithubRepository } from "../../models/repository.ts"

export function convertDatasetFromGithub(githubRepository: GithubRepository): Dataset {
  const dataset: Dataset = {
    resources: [],
  }

  if (githubRepository.full_name) {
    dataset.titles = [{ title: githubRepository.full_name }]
  }

  if (githubRepository.description) {
    dataset.descriptions = [
      {
        description: githubRepository.description,
        descriptionType: "Abstract",
      },
    ]
  }

  if (githubRepository.license) {
    dataset.rightsList = [
      {
        rights: githubRepository.license.name,
        rightsUri: githubRepository.license.url,
        rightsIdentifier:
          githubRepository.license.spdx_id || githubRepository.license.key,
        rightsIdentifierScheme: "SPDX",
      },
    ]
  }

  if (githubRepository.owner) {
    const contributor = {
      name: githubRepository.owner.login,
      nameType:
        githubRepository.owner.type === "Organization" ? "Organizational" : "Personal",
    } as const

    if (githubRepository.owner.type === "Organization") {
      dataset.contributors = [
        {
          ...contributor,
          contributorType: "HostingInstitution",
        },
      ]
    } else {
      dataset.creators = [contributor]
    }
  }

  if (githubRepository.files && githubRepository.files.length > 0) {
    dataset.resources = githubRepository.files
      .filter(file => !file.path.startsWith("."))
      .filter(file => file.type === "blob")
      .map(file =>
        convertResourceFromGithub(file, {
          defaultBranch: githubRepository.default_branch,
        }),
      )
  }

  if (githubRepository.topics && githubRepository.topics.length > 0) {
    dataset.subjects = githubRepository.topics.map(topic => ({
      subject: topic,
    }))
  }

  if (githubRepository.created_at) {
    dataset.dates = [
      {
        date: githubRepository.created_at,
        dateType: "Created",
      },
    ]
  }

  return dataset
}
