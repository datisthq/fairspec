import type { Descriptor } from "@fairspec/metadata"
import { z } from "zod"

/**
 * Makes a request to the Github API
 */
export async function makeGithubApiRequest<S extends z.ZodType>(options: {
  endpoint: string
  method?: "GET" | "POST" | "PUT" | "DELETE" | "PATCH"
  payload?: Descriptor
  apiKey?: string
  upload?: {
    name: string
    data: Blob
    path?: string // Path within repository
  }
  schema: S
}) {
  const { endpoint, method = "GET", payload, upload, apiKey } = options

  let body: string | FormData | undefined
  const headers: Record<string, any> = {}

  if (apiKey) {
    headers["Authorization"] = `Bearer ${apiKey}`
  }

  // Create full API URL
  const baseUrl = "https://api.github.com"
  const url = `${baseUrl}${endpoint}`

  if (upload) {
    body = new FormData()
    body.append("file", upload.data, upload.name)

    if (payload) {
      for (const [key, value] of Object.entries(payload)) {
        if (typeof value === "object" && value !== null) {
          body.append(key, JSON.stringify(value))
        } else {
          body.append(key, String(value))
        }
      }
    }
  } else if (payload) {
    body = JSON.stringify(payload)
    headers["Content-Type"] = "application/json"
  }

  const response = await fetch(url, {
    method,
    headers,
    body,
  })

  if (!response.ok || response.status === 204) {
    const errorText = await response.text()
    throw new Error(
      `Github API error: ${response.status} ${response.statusText}\n${errorText}`,
    )
  }

  const result = options.schema.safeParse(await response.json())
  if (!result.success) {
    throw new Error(
      `Github API returned an unexpected response from ${endpoint}:\n${z.prettifyError(result.error)}`,
    )
  }

  return result.data
}
