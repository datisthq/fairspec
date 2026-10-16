import { loadFileStream } from "../../actions/stream/load.ts"
import { saveFileStream } from "../../actions/stream/save.ts"

export async function copyFile(options: {
  sourcePath: string
  targetPath: string
  maxBytes?: number
}) {
  const stream = await loadFileStream(options.sourcePath, {
    maxBytes: options.maxBytes,
  })

  await saveFileStream(stream, { path: options.targetPath })
}
