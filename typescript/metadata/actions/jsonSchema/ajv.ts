import { Ajv2020 } from "ajv/dist/2020.js"
import { loadJsonSchema } from "./load.ts"

export const ajv = new Ajv2020({
  strict: false,
  allErrors: true,
  validateSchema: false,
  validateFormats: false,
  loadSchema: uri => loadJsonSchema(uri, { onlyRemote: true }),
})
