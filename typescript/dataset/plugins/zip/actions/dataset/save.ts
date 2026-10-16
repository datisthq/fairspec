import { Buffer } from "node:buffer"
import { writeFile } from "node:fs/promises"
import type { Readable } from "node:stream"
import type { Dataset, Descriptor } from "@fairspec/metadata"
import { denormalizeDataset, stringifyDescriptor } from "@fairspec/metadata"
import { zip } from "fflate"
import { getDatasetBasepath } from "../../../../actions/dataset/basepath.ts"
import { assertLocalPathVacant } from "../../../../actions/file/path.ts"
import { saveResourceFiles } from "../../../../actions/resource/save.ts"
import { loadFileStream } from "../../../../actions/stream/load.ts"

export async function saveDatasetToZip(
  dataset: Dataset,
  options: {
    archivePath: string
    withRemote?: boolean
  },
) {
  const { archivePath, withRemote } = options
  const basepath = getDatasetBasepath(dataset)

  await assertLocalPathVacant(archivePath)
  const files: Record<string, Uint8Array> = {}

  const resourceDescriptors: Descriptor[] = []
  for (const resource of dataset.resources ?? []) {
    resourceDescriptors.push(
      await saveResourceFiles(resource, {
        basepath,
        withRemote,
        saveFile: async options => {
          const stream = await loadFileStream(options.normalizedPath)
          const buffer = await streamToBuffer(stream)
          files[options.denormalizedPath] = buffer

          return options.denormalizedPath
        },
      }),
    )
  }

  const descriptor = {
    ...denormalizeDataset(dataset, { basepath }),
    resources: resourceDescriptors,
  }

  files["dataset.json"] = Buffer.from(stringifyDescriptor(descriptor))

  const zipData = await new Promise<Uint8Array>((resolve, reject) => {
    zip(files, (err, data) => {
      if (err) reject(err)
      else resolve(data)
    })
  })

  await writeFile(archivePath, zipData)
}

async function streamToBuffer(stream: Readable) {
  const chunks: Uint8Array[] = []
  for await (const chunk of stream) {
    chunks.push(chunk)
  }
  return Buffer.concat(chunks)
}
