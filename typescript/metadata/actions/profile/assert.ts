import type { JsonSchema } from "../../models/jsonSchema.ts"
import type { Profile, ProfileType } from "../../models/profile.ts"

export async function assertProfile(
  jsonSchema: JsonSchema,
  options: {
    path: string
    type: ProfileType
  },
) {
  const regex = new RegExp(
    `^https:\\/\\/fairspec\\.org\\/profiles\\/(\\d+\\.\\d+\\.\\d+|latest)\\/${options.type}\\.json$`,
  )

  const paths = [options.path, ...getAllOfPaths(jsonSchema)]

  // Data schema supports direct JSON schema profile
  if (options.type === "data-schema") {
    if (options.path === "https://json-schema.org/draft/2020-12/schema") {
      return jsonSchema as Profile
    }
  }

  // If one of the paths matches the profile type, we're good
  for (const path of paths) {
    if (regex.test(path)) {
      return jsonSchema as Profile
    }
  }

  throw new Error(
    `Profile at path ${options.path} is not a valid ${options.type} profile`,
  )
}

function getAllOfPaths(jsonSchema: JsonSchema) {
  if (!Array.isArray(jsonSchema.allOf)) return []
  return jsonSchema.allOf.flatMap((item: unknown) => {
    if (typeof item === "string") return [item]
    if (
      typeof item === "object" &&
      item &&
      "$ref" in item &&
      typeof item.$ref === "string"
    ) {
      return [item.$ref]
    }
    return []
  })
}
