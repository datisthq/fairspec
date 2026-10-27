import { describe, expect, it } from "vite-plus/test"
import { inspectDataSchema } from "../inspect.ts"

describe("inspectDataSchema", () => {
  it("accepts the 2020-12 dialect and a Fairspec profile", () => {
    for (const $schema of [
      "https://json-schema.org/draft/2020-12/schema",
      "https://fairspec.org/profiles/1.0.0/data-schema.json",
    ]) {
      expect(inspectDataSchema({ $schema })).toEqual([])
    }
  })

  it("reports another dialect at $schema", () => {
    const errors = inspectDataSchema(
      { $schema: "http://json-schema.org/draft-07/schema#" },
      { rootJsonPointer: "/dataSchema" },
    )

    expect(errors.map(error => error.jsonPointer)).toEqual(["/dataSchema/$schema"])
  })

  it("accepts references that resolve within the document", () => {
    const errors = inspectDataSchema({
      $defs: { "a/b": { type: "string" }, "c d": { $anchor: "cd" } },
      items: [{ $ref: "#/$defs/a~1b" }, { $ref: "#/$defs/c%20d" }, { $ref: "#cd" }],
      not: { $ref: "#" },
    })

    expect(errors).toEqual([])
  })

  it("reports every reference that leaves the document, in document order", () => {
    const errors = inspectDataSchema({
      properties: {
        a: { $ref: "https://example.com/schema.json" },
        b: { $ref: "#/$defs/missing" },
      },
      items: { $ref: "#/$defs/%ZZ" },
      const: { $ref: "https://example.com/data-not-schema" },
    })

    expect(errors.map(error => error.jsonPointer)).toEqual([
      "/properties/a/$ref",
      "/properties/b/$ref",
      "/items/$ref",
    ])
  })
})
