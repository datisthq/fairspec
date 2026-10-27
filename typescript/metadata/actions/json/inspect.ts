import { Ajv2020, type ValidateFunction } from "ajv/dist/2020.js"
import { META_SCHEMA_URL, ajv } from "../../actions/jsonSchema/ajv.ts"
import { loadJsonSchema } from "../../actions/jsonSchema/load.ts"
import type { JsonSchema } from "../../models/jsonSchema.ts"

/**
 * Validate a value against a JSON Schema
 * It uses Ajv for JSON Schema validation under the hood
 */
export async function inspectJson(
  value: unknown,
  options: {
    jsonSchema: JsonSchema | string
    rootJsonPointer?: string
    formats?: Record<string, (value: string) => boolean>
  },
) {
  const jsonSchema =
    typeof options.jsonSchema === "string"
      ? await loadJsonSchema(options.jsonSchema)
      : options.jsonSchema

  const metaSchema =
    jsonSchema.$id === META_SCHEMA_URL ? ajv.getSchema(META_SCHEMA_URL) : undefined
  const validate =
    metaSchema ??
    (options.formats
      ? getFormatValidator(jsonSchema, options.formats)
      : await ajv.compileAsync(jsonSchema))
  validate(value)

  // An `if` failure only repeats, at the parent, the error its `then` or `else` reports
  const errors = (validate.errors ?? [])
    .filter(error => error.keyword !== "if")
    .map(error => {
      const instancePath = error.instancePath ?? "/"
      const rootPath = options.rootJsonPointer ?? ""
      const jsonPointer =
        rootPath === "" || rootPath === "/"
          ? instancePath
          : instancePath === "/"
            ? rootPath
            : `${rootPath}${instancePath}`

      return {
        message: error.message ?? "error",
        jsonPointer,
      }
    })

  return errors
}

const formatValidators = new WeakMap<
  JsonSchema,
  { formats: object; validate: ValidateFunction }
>()

// Only the given formats are asserted, an unknown one stays an annotation; a schema
// checked with formats is data, whose references never leave the document. One Ajv per
// schema object: a second object with the same `$id` throws in a shared instance
function getFormatValidator(
  jsonSchema: JsonSchema,
  formats: Record<string, (value: string) => boolean>,
) {
  const cached = formatValidators.get(jsonSchema)
  if (cached?.formats === formats) return cached.validate

  const formatAjv = new Ajv2020({
    strict: false,
    allErrors: true,
    validateSchema: false,
    validateFormats: true,
    formats,
    logger: false,
  })
  const validate = formatAjv.compile(jsonSchema)
  formatValidators.set(jsonSchema, { formats, validate })
  return validate
}
