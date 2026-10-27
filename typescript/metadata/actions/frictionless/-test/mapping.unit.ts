import { describe, expect, it } from "vite-plus/test"
import { convertDescriptorFromFrictionless } from "../convert.ts"
import { mapFrictionlessErrors } from "../mapping.ts"

describe("mapFrictionlessErrors", () => {
  const { mapping } = convertDescriptorFromFrictionless(
    {
      resources: [
        {
          name: "fruit-list",
          path: "fruit.csv",
          schema: {
            fields: [
              { name: "id", type: "integer" },
              { name: "day", type: "date", format: "%d/%m/%Y" },
            ],
            missingValues: ["", "NA"],
          },
        },
      ],
    },
    { kind: "dataset" },
  )

  it("should point at the field and key a property came from", () => {
    const errors = mapFrictionlessErrors(
      [
        {
          type: "metadata",
          message: "",
          jsonPointer: "/resources/0/tableSchema/properties/day/temporalFormat",
        },
        {
          type: "metadata",
          message: "",
          jsonPointer: "/resources/0/tableSchema/properties/day",
        },
      ],
      mapping,
    )

    expect(
      errors.map(error => ("jsonPointer" in error ? error.jsonPointer : "")),
    ).toEqual(["/resources/0/schema/fields/1/format", "/resources/0/schema/fields/1"])
  })

  it("should keep the key path below a value copied as it is", () => {
    const [error] = mapFrictionlessErrors(
      [
        {
          type: "metadata",
          message: "",
          jsonPointer: "/resources/0/tableSchema/missingValues/1",
        },
      ],
      mapping,
    )

    expect(error).toMatchObject({ jsonPointer: "/resources/0/schema/missingValues/1" })
  })

  it("should name resources as the package does", () => {
    const [error] = mapFrictionlessErrors(
      [
        {
          type: "cell/type",
          columnName: "id",
          columnType: "integer",
          rowNumber: 1,
          cell: "x",
          resourceName: "fruit_list",
        },
      ],
      mapping,
    )

    expect(error).toMatchObject({ resourceName: "fruit-list" })
  })
})
