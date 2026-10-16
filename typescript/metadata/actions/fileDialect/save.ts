import { copyDescriptor } from "../../actions/descriptor/copy.ts"
import { saveDescriptor } from "../../actions/descriptor/save.ts"
import type { FileDialect } from "../../models/fileDialect/fileDialect.ts"
import * as settings from "../../settings.ts"

export async function saveFileDialect(
  fileDialect: FileDialect,
  options: {
    path: string
    overwrite?: boolean
  },
) {
  const descriptor = copyDescriptor(fileDialect)

  descriptor.$schema =
    descriptor.$schema ??
    `https://fairspec.org/profiles/${settings.FAIRSPEC_VERSION}/file-dialect.json`

  await saveDescriptor(descriptor, {
    path: options.path,
    overwrite: options.overwrite,
  })
}
