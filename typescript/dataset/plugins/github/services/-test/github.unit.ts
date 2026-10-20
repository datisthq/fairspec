import { afterEach, describe, expect, it, vi } from "vite-plus/test"
import { GithubTree } from "../../models/tree.ts"
import { makeGithubApiRequest } from "../github.ts"

describe("makeGithubApiRequest", () => {
  afterEach(() => {
    vi.restoreAllMocks()
  })

  it("returns the response parsed by the schema", async () => {
    const entry = {
      path: "data",
      mode: "040000",
      type: "tree",
      sha: "abc",
      url: "https://api.github.com/repos/o/r/git/trees/abc",
    }
    mockFetchJson({ sha: "def", tree: [entry], truncated: false })

    const tree = await makeGithubApiRequest({
      endpoint: "/repos/o/r/git/trees/main",
      schema: GithubTree,
    })

    expect(tree).toEqual({ tree: [entry] })
  })

  it("rejects a response that does not match the schema", async () => {
    mockFetchJson({ message: "Not Found" })

    await expect(
      makeGithubApiRequest({
        endpoint: "/repos/o/r/git/trees/main",
        schema: GithubTree,
      }),
    ).rejects.toThrow(
      /Github API returned an unexpected response from \/repos\/o\/r\/git\/trees\/main[\s\S]*tree/,
    )
  })
})

function mockFetchJson(body: unknown) {
  vi.spyOn(globalThis, "fetch").mockResolvedValue(new Response(JSON.stringify(body)))
}
