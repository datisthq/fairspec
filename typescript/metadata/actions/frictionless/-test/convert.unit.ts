import { describe, expect, it } from "vite-plus/test"
import {
  convertDescriptorFromFrictionless,
  getIsFrictionlessDescriptor,
} from "../convert.ts"

describe("getIsFrictionlessDescriptor", () => {
  it("should recognise a schema by its fields", () => {
    expect(getIsFrictionlessDescriptor({ fields: [] }, { kind: "tableSchema" })).toBe(
      true,
    )
    expect(getIsFrictionlessDescriptor({ properties: {} }, { kind: "tableSchema" })).toBe(
      false,
    )
  })

  it("should recognise a dialect without a format", () => {
    expect(getIsFrictionlessDescriptor({ delimiter: ";" }, { kind: "fileDialect" })).toBe(
      true,
    )
    expect(getIsFrictionlessDescriptor({ format: "csv" }, { kind: "fileDialect" })).toBe(
      false,
    )
  })

  it("should recognise a package by its resources or profile", () => {
    const isPackage = (descriptor: Record<string, unknown>) =>
      getIsFrictionlessDescriptor(descriptor, { kind: "dataset" })

    expect(isPackage({ resources: [{ path: "a.csv" }] })).toBe(true)
    expect(isPackage({ profile: "data-package", resources: [] })).toBe(true)
    expect(isPackage({ resources: [{ data: "a.csv" }] })).toBe(false)
  })

  it("should never take a Fairspec descriptor for a Frictionless one", () => {
    const descriptor = {
      $schema: "https://fairspec.org/profiles/latest/table-schema.json",
      fields: [],
    }
    expect(getIsFrictionlessDescriptor(descriptor, { kind: "tableSchema" })).toBe(false)
  })
})

describe("convertDescriptorFromFrictionless", () => {
  it("should report a descriptor the model cannot read", () => {
    const result = convertDescriptorFromFrictionless(
      { fields: [{ name: 1 }] },
      { kind: "tableSchema" },
    )

    expect(result.descriptor).toBeUndefined()
    expect(result.errors).toEqual([
      expect.objectContaining({ type: "metadata", jsonPointer: "/fields/0/name" }),
    ])
  })
})
