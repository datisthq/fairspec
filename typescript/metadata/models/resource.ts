import { z } from "zod"
import { Data } from "./data.ts"
import { Datacite } from "./datacite/datacite.ts"
import { DataSchema } from "./dataSchema.ts"
import { FileDialect } from "./fileDialect/fileDialect.ts"
import { Integrity } from "./integrity.ts"
import { Path } from "./path.ts"
import { TableSchema } from "./tableSchema.ts"

export const Resource = Datacite.extend({
  data: Data.optional().describe(
    "Data or content of the resource. It must be a path to a file, array of paths to files, inline JSON object, or inline JSON array of objects.",
  ),

  name: z
    .string()
    .regex(/^[a-zA-Z0-9_]+$/)
    .optional()
    .describe(
      "An optional name for the resource consisting of alphanumeric characters and underscores. If provided, it can be used to reference resource within a dataset context.",
    ),

  textual: z.boolean().optional().describe("Whether the resource is text-based."),

  integrity: Integrity.optional().describe(
    "The integrity check of the file with type (md5, sha1, sha256, sha512) and hash value.",
  ),

  fileDialect: z
    .union([Path, FileDialect])
    .optional()
    .describe(
      "A path to dialect or an object with the dialect of the file. For multiple files the format property defines the dialect for all the files.",
    ),

  dataSchema: z
    .union([Path, DataSchema])
    .optional()
    .describe(
      "A path to a JSON Schema or an object with the JSON Schema. The JSON Schema must be compatible with the JSONSchema Draft 2020-12 specification.",
    ),

  tableSchema: z
    .union([Path, TableSchema])
    .optional()
    .describe(
      "A path to a Table Schema or an object with the Table Schema. The Table Schema must be compatible with the Fairspec Table specification.",
    ),

  // TODO: remove temporary solution for dataset plugins
  unstable_customMetadata: z
    .record(z.string(), z.unknown())
    .optional()
    .describe("Custom properties for extending resources"),
})

export type Resource = z.infer<typeof Resource>
