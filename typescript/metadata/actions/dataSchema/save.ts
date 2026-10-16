import { copyDescriptor } from "../../actions/descriptor/copy.ts"
import { saveDescriptor } from "../../actions/descriptor/save.ts"
import type { DataSchema } from "../../models/dataSchema.ts"
import * as settings from "../../settings.ts"

export async function saveDataSchema(
  dataSchema: DataSchema,
  options: {
    path: string
    overwrite?: boolean
  },
) {
  const descriptor = copyDescriptor(dataSchema)

  descriptor.$schema =
    descriptor.$schema ??
    `https://fairspec.org/profiles/${settings.FAIRSPEC_VERSION}/data-schema.json`

  await saveDescriptor(descriptor, {
    path: options.path,
    overwrite: options.overwrite,
  })
}
