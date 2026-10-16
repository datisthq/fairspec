import { readFile, readdir } from "node:fs/promises"
import { join } from "node:path"
import { Ajv2020 } from "ajv/dist/2020.js"
import ajvFormats from "ajv-formats"
import { describe, expect, it } from "vite-plus/test"

const STANDARD = join(import.meta.dirname, "..")
const BASE_URL = "https://fairspec.org/profiles/latest/"

const profileNames = await listProfiles()
const ajv = await createAjv()
const specExamples = await Promise.all(
  ["data-schema", "table-schema"].map(async spec => ({
    spec,
    examples: await readJsonExamples(spec),
  })),
)

describe("profiles", () => {
  it.each(profileNames)("compiles %s", name => {
    expect(ajv.getSchema(BASE_URL + name)).toBeTypeOf("function")
  })
})

describe.each(specExamples)("$spec spec examples", ({ spec, examples }) => {
  it.each(examples)("validates the example at line $line", example => {
    const validate = getValidator(`${spec}.json`)
    expect(validate(JSON.parse(example.text)), JSON.stringify(validate.errors)).toBe(true)
  })
})

describe("table schema columns", () => {
  it.each([
    { valid: true, column: { type: "string", minLength: 1 } },
    { valid: true, column: { type: ["null", "string"], format: "date" } },
    { valid: true, column: { type: "array", items: { type: "number" } } },
    { valid: true, column: { type: "object", format: "geojson" } },
    { valid: false, column: { type: "string", enum: [1, 2] } },
    { valid: false, column: { type: "string", format: "bogus" } },
    { valid: false, column: { type: "array", const: "x" } },
    { valid: false, column: { type: "object", format: "bogus" } },
  ])("validates $column as $valid", ({ column, valid }) => {
    const validate = getValidator("table-schema.json")
    expect(validate({ properties: { column } })).toBe(valid)
  })
})

async function listProfiles() {
  const entries = await readdir(join(STANDARD, "profiles"), { recursive: true })
  return entries.filter(entry => entry.endsWith(".json")).sort()
}

async function createAjv() {
  const instance = new Ajv2020({ strict: false, allErrors: true, logger: false })
  ajvFormats.default(instance)
  for (const name of profileNames) {
    const text = await readFile(join(STANDARD, "profiles", name), "utf8")
    const resolved = text.replace(/\{([a-z-]+)-ref\}/g, `${BASE_URL}$1.json`)
    instance.addSchema(JSON.parse(resolved), BASE_URL + name)
  }
  return instance
}

function getValidator(name: string) {
  const validate = ajv.getSchema(BASE_URL + name)
  if (!validate) throw new Error(`Profile not found: ${name}`)
  return validate
}

async function readJsonExamples(spec: string) {
  const text = await readFile(join(STANDARD, "specs", `${spec}.md`), "utf8")
  return [...text.matchAll(/```json\n([\s\S]*?)```/g)].map(match => ({
    line: text.slice(0, match.index).split("\n").length,
    text: match[1] ?? "",
  }))
}
