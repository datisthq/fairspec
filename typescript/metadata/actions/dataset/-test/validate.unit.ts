import { describe, expect, it } from "vite-plus/test"
import { useRecording } from "../../../-test/recording.ts"
import { validateDatasetDescriptor } from "../validate.ts"

useRecording()

describe("validateDatasetDescriptor", () => {
  it("reports what the profile accepts but the model rejects", async () => {
    const dataset = {
      resources: [
        { data: "data.csv", tableSchema: { $schema: "http://", properties: {} } },
      ],
    }

    const report = await validateDatasetDescriptor(dataset)

    expect(report.valid).toBe(false)
    expect(report.dataset).toBe(undefined)
    expect(report.errors).toEqual([
      expect.objectContaining({
        type: "metadata",
        jsonPointer: "/resources/0/tableSchema/$schema",
      }),
    ])
  })

  it("keeps properties an extension adds on top of the core model", async () => {
    const dataset = { resources: [{ data: "data.csv", spectralRange: { min: 400 } }] }

    const report = await validateDatasetDescriptor(dataset)

    expect(report.dataset?.resources?.[0]).toEqual(
      expect.objectContaining({ spectralRange: { min: 400 } }),
    )
  })

  it("returns valid result for valid dataset", async () => {
    const dataset = {
      resources: [
        {
          data: "data.csv",
        },
      ],
    }

    const report = await validateDatasetDescriptor(dataset)

    expect(report.valid).toBe(true)
    expect(report.errors).toEqual([])
  })

  it("returns validation errors for invalid dataset", async () => {
    const dataset = {
      resources: "not-an-array",
    }

    const report = await validateDatasetDescriptor(dataset)

    expect(report.valid).toBe(false)
    expect(report.errors.length).toBeGreaterThan(0)
  })

  it("dont return validation error for missing schema", async () => {
    const dataset = {
      resources: [
        {
          data: "data.csv",
        },
      ],
    }

    const report = await validateDatasetDescriptor(dataset)
    expect(report.valid).toBe(true)
  })

  it("validates dataset with resource integrity", async () => {
    const dataset = {
      resources: [
        {
          data: "data.csv",
          integrity: {
            type: "sha256",
            hash: "5f951fd841d3bd6c03ea05024f8c564096e10a3e17483e6a70d25a885942683a",
          },
        },
      ],
    }

    const report = await validateDatasetDescriptor(dataset)

    expect(report.valid).toBe(true)
    expect(report.errors).toEqual([])
  })

  it("validates dataset with datacite metadata", async () => {
    const dataset = {
      creators: [
        {
          name: "John Doe",
        },
      ],
      titles: [
        {
          title: "Example Dataset",
        },
      ],
      resources: [
        {
          data: "data.csv",
        },
      ],
    }

    const report = await validateDatasetDescriptor(dataset)

    expect(report.valid).toBe(true)
    expect(report.errors).toEqual([])
  })
})
