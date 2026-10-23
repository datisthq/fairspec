import type { Descriptor } from "@fairspec/metadata"
import { fetchRemote } from "@fairspec/metadata"
import { z } from "zod"

export async function makeCkanApiRequest<S extends z.ZodType>(options: {
  ckanUrl: string
  action: string
  payload: Descriptor
  upload?: { name: string; data: Blob }
  apiKey?: string
  schema: S
}) {
  let body: string | FormData
  const headers: Record<string, any> = {}

  const url = new URL(options.ckanUrl)
  url.pathname = `/api/3/action/${options.action}`

  if (options.apiKey) {
    headers.Authorization = options.apiKey
  }

  if (options.upload) {
    body = new FormData()
    body.append("upload", options.upload.data, options.upload.name)

    for (const [key, value] of Object.entries(options.payload)) {
      body.append(key, value)
    }
  } else {
    body = JSON.stringify(options.payload)
    headers["Content-Type"] = "application/json"
  }

  const response = await fetchRemote(url.toString(), {
    method: "POST",
    headers,
    body,
  })

  if (!response.ok) {
    const errorText = await response.text()
    throw new Error(
      `CKAN API error: ${response.status} ${response.statusText}\n${errorText}`,
    )
  }

  const envelope = CkanResponse.safeParse(await response.json())
  if (!envelope.success) {
    throw createResponseError(options.action, envelope.error)
  }

  if (!envelope.data.success) {
    const { error } = envelope.data
    throw new Error(`CKAN API error: ${error?.message ?? JSON.stringify(error)}`)
  }

  const result = options.schema.safeParse(envelope.data.result)
  if (!result.success) {
    throw createResponseError(options.action, result.error)
  }

  return result.data
}

const CkanResponse = z.object({
  success: z.boolean(),
  result: z.unknown().optional(),
  error: z.looseObject({ message: z.string().optional() }).optional(),
})

function createResponseError(action: string, error: z.ZodError) {
  return new Error(
    `CKAN API returned an unexpected response from ${action}:\n${z.prettifyError(error)}`,
  )
}
