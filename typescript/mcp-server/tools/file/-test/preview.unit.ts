import { writeTempFile } from "@fairspec/library"
import { describe, expect, it } from "vite-plus/test"
import { previewFileTool } from "../preview.ts"

describe("previewFileTool", () => {
  it("validates tool structure", () => {
    expect(previewFileTool.name).toBe("previewFileTool")
    expect(previewFileTool.description).toBeTruthy()
    expect(previewFileTool.inputSchema).toBeTruthy()
    expect(previewFileTool.outputSchema).toBeTruthy()
    expect(previewFileTool.execute).toBeTypeOf("function")
  })

  it("previews a CSV file", async () => {
    const path = await writeTempFile("id,name\n1,alice\n2,bob\n")

    const result = await previewFileTool.execute({ path, maxLines: 2 })

    expect(result).toEqual({ text: "id,name\n1,alice\n", truncated: true })
  })

  it("caps the preview limits", () => {
    const { inputSchema } = previewFileTool

    expect(inputSchema.safeParse({ path: "a.csv", maxLines: 200 }).success).toBe(true)
    expect(inputSchema.safeParse({ path: "a.csv", maxLines: 201 }).success).toBe(false)
    expect(inputSchema.safeParse({ path: "a.csv", maxBytes: 20_001 }).success).toBe(false)
    expect(inputSchema.safeParse({ path: "a.csv", maxBytes: 0 }).success).toBe(false)
  })
})
