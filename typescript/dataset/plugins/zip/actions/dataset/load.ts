import { readFile, writeFile } from "node:fs/promises"
import { mkdir } from "node:fs/promises"
import { dirname, join } from "node:path"
import { loadDatasetDescriptor } from "@fairspec/metadata"
import { getTempFolderPath } from "../../../../actions/folder/temp.ts"
import { unzip } from "fflate"

export async function loadDatasetFromZip(archivePath: string) {
  const basepath = getTempFolderPath()
  const zipData = await readFile(archivePath)

  const entries = await new Promise<Record<string, Uint8Array>>((resolve, reject) => {
    unzip(zipData, (err, unzipped) => {
      if (err) reject(err)
      else resolve(unzipped)
    })
  })

  for (const [filename, data] of Object.entries(entries)) {
    const path = join(basepath, filename)

    if (filename.endsWith("/")) {
      await mkdir(path, { recursive: true })
      continue
    }

    await mkdir(dirname(path), { recursive: true })
    await writeFile(path, data)
  }

  const dataset = await loadDatasetDescriptor(join(basepath, "dataset.json"))

  return dataset
}
