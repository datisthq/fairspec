import { writeTempFile } from "@fairspec/library"
import { afterEach, beforeEach, describe, expect, it, vi } from "vite-plus/test"
import { queryDatasetCommand } from "../query.ts"

describe("dataset query", () => {
  beforeEach(() => {
    vi.spyOn(process.stdout, "write").mockImplementation(() => true)
    vi.spyOn(process.stderr, "write").mockImplementation(() => true)
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it("should join two resources by name", async () => {
    const descriptor = {
      resources: [
        {
          name: "users",
          data: [
            { id: 1, name: "alice" },
            { id: 2, name: "bob" },
          ],
        },
        {
          name: "posts",
          data: [
            { id: 1, user_id: 2 },
            { id: 2, user_id: 2 },
          ],
        },
      ],
    }
    const descriptorPath = await writeTempFile(JSON.stringify(descriptor), {
      format: "json",
    })

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await queryDatasetCommand.parseAsync(
        [
          descriptorPath,
          "SELECT users.name, COUNT(*) AS n FROM posts JOIN users ON posts.user_id = users.id GROUP BY users.name",
          "--json",
        ],
        { from: "user" },
      )
    } catch {}

    expect(JSON.parse(text[0] ?? "")).toEqual([{ name: "bob", n: 2 }])
  })
})
