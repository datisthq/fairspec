import { copyDescriptor } from "../../actions/descriptor/copy.ts"
import { saveDescriptor } from "../../actions/descriptor/save.ts"
import type { TableSchema } from "../../models/tableSchema.ts"
import * as settings from "../../settings.ts"

export async function saveTableSchema(
  tableSchema: TableSchema,
  options: {
    path: string
    overwrite?: boolean
  },
) {
  const descriptor = copyDescriptor(tableSchema)

  descriptor.$schema =
    descriptor.$schema ??
    `https://fairspec.org/profiles/${settings.FAIRSPEC_VERSION}/table-schema.json`

  await saveDescriptor(descriptor, {
    path: options.path,
    overwrite: options.overwrite,
  })
}
