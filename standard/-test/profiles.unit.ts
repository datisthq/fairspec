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
    examples: await readJsonBlocks(join("specs", `${spec}.md`)),
  })),
)
const documentExamples = await readDocumentExamples()

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

describe.each(documentExamples)("$document examples", ({ examples }) => {
  it.each(examples)("validates the $profile example at line $line", example => {
    const validate = getValidator(example.profile)
    expect(validate(JSON.parse(example.text)), JSON.stringify(validate.errors)).toBe(true)
  })
})

describe("table schemas", () => {
  it.each([
    { valid: true, schema: { $schema: "https://json-schema.org/draft/2020-12/schema" } },
    { valid: true, schema: { type: "object", additionalProperties: false } },
    { valid: true, schema: { additionalProperties: true, $comment: "note" } },
    { valid: false, schema: { additionalProperties: { type: "string" } } },
    { valid: false, schema: { oneOf: [{ required: ["a"] }] } },
    { valid: false, schema: { $defs: { a: { type: "string" } } } },
  ])("validates $schema as $valid", ({ schema, valid }) => {
    const validate = getValidator("table-schema.json")
    expect(validate(schema)).toBe(valid)
  })
})

describe.each(["datacite", "grei"])("%s extension", extension => {
  const required = {
    $schema: `${BASE_URL}${extension}/dataset.json`,
    doi: "10.1234/5678",
    creators: [{ name: "Jane" }],
    titles: [{ title: "T" }],
    publisher: { name: "P" },
    publicationYear: "2024",
    types: { resourceTypeGeneral: "Dataset" },
    subjects: [{ subject: "S" }],
    dates: [{ date: "2024", dateType: "Created" }],
    relatedIdentifiers: [
      {
        relatedIdentifier: "10.1/x",
        relatedIdentifierType: "DOI",
        relationType: "Cites",
      },
    ],
    version: "1",
    rightsList: [{ rights: "CC0" }],
    descriptions: [{ description: "D", descriptionType: "Abstract" }],
    fundingReferences: [{ funderName: "F" }],
  }

  it.each([
    { valid: true, note: "the required properties with types", descriptor: required },
    {
      valid: false,
      note: "resourceType in place of types",
      descriptor: { ...required, types: undefined, resourceType: "Dataset" },
    },
    {
      valid: false,
      note: "another profile as $schema",
      descriptor: { ...required, $schema: "https://example.com/dataset.json" },
    },
  ])("validates $note as $valid", ({ descriptor, valid }) => {
    const validate = getValidator(`${extension}/dataset.json`)
    expect(validate(JSON.parse(JSON.stringify(descriptor)))).toBe(valid)
  })
})

describe("examples", () => {
  it.each(["dataset", "table-schema"])("validates %s.json", async name => {
    const text = await readFile(join(STANDARD, "examples", `${name}.json`), "utf8")
    const validate = getValidator(`${name}.json`)
    expect(validate(JSON.parse(text)), JSON.stringify(validate.errors)).toBe(true)
  })

  it("validates catalog.jsonl", async () => {
    const text = await readFile(join(STANDARD, "examples", "catalog.jsonl"), "utf8")
    const lines = text.split(/\r?\n/).filter(line => line.trim())
    const validate = getValidator("catalog.json")
    const catalog = lines.map(line => JSON.parse(line))
    expect(validate(catalog), JSON.stringify(validate.errors)).toBe(true)
  })
})

describe("table schema columns", () => {
  it.each([
    { valid: true, column: { type: "string", minLength: 1 } },
    { valid: true, column: { type: ["null", "string"], format: "date" } },
    { valid: true, column: { type: "array", items: { type: "number" } } },
    { valid: true, column: { type: "object", format: "geojson" } },
    { valid: false, column: { type: "string", enum: [1, 2] } },
    { valid: true, column: { type: "string", format: "bogus" } },
    { valid: false, column: { type: "array", const: "x" } },
    { valid: true, column: { type: "object", format: "bogus" } },
    { valid: true, column: { default: "x", const: "x", examples: ["x", 1] } },
    { valid: true, column: { enum: ["a", 1, true] } },
    { valid: true, column: { missingValues: ["NA"] } },
    { valid: true, column: { type: "string", default: "x" } },
    { valid: false, column: { type: "integer", default: "x" } },
    { valid: true, column: { type: ["integer"], minimum: 1 } },
    { valid: true, column: { type: ["null", "string"], enum: ["a", null] } },
    { valid: true, column: { type: ["integer", "number"], minimum: 0.5 } },
    { valid: true, column: { type: "integer", format: "int64" } },
    { valid: true, column: { type: "array", items: { anyOf: [{ type: "string" }] } } },
    { valid: false, column: { type: ["integer", "string"] } },
    { valid: false, column: { type: "date" } },
    { valid: false, column: { type: "string", enum: [1] } },
    { valid: false, column: { anyOf: [{ type: "string" }] } },
    { valid: false, column: { type: "string", $ref: "#/$defs/x" } },
    { valid: false, column: { type: "string", format: "list", delimiter: 5 } },
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

async function readJsonBlocks(document: string) {
  const text = await readFile(join(STANDARD, document), "utf8")
  return [...text.matchAll(/```json\n([\s\S]*?)```/g)].map(match => ({
    line: text.slice(0, match.index).split("\n").length,
    text: match[1] ?? "",
  }))
}

async function readDocumentExamples() {
  const entries = await readdir(STANDARD, { recursive: true })
  const documents = entries
    .filter(entry => entry.endsWith(".md") && !entry.includes("node_modules"))
    .sort()
  const profile = /"\$schema": "https:\/\/fairspec\.org\/profiles\/[^/]+\/([^"]+)"/
  const results = []
  for (const document of documents) {
    const examples = []
    for (const block of await readJsonBlocks(document)) {
      const name = profile.exec(block.text)?.[1]
      if (name) examples.push({ ...block, profile: name })
    }
    if (examples.length) results.push({ document, examples })
  }
  return results
}
