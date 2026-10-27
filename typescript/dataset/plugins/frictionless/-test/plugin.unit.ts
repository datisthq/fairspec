import { describe, expect, it } from "vite-plus/test"
import { FrictionlessPlugin } from "../plugin.ts"

describe("FrictionlessPlugin", () => {
  const plugin = new FrictionlessPlugin()

  it("should not save to a remote datapackage.json", async () => {
    const result = await plugin.saveDataset(
      { resources: [] },
      { target: "https://example.com/datapackage.json" },
    )
    expect(result).toBeUndefined()
  })

  it("should not save to a json file with another name", async () => {
    const result = await plugin.saveDataset(
      { resources: [] },
      { target: "./dataset.json" },
    )
    expect(result).toBeUndefined()
  })
})
