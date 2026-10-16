import { existsSync } from "node:fs"
import { readFile, readdir } from "node:fs/promises"
import { join, resolve } from "node:path"
import { describe, expect, it } from "vite-plus/test"
import standardPackage from "../../standard/package.json" with { type: "json" }

const ROOT = resolve(import.meta.dirname, "..", "..")
const PUBLISHED = join(ROOT, ".livemark", "public", "profiles")
const VERSION = standardPackage.version

describe("published profiles", () => {
  it("include the current standard version", () => {
    expect(existsSync(join(PUBLISHED, VERSION))).toBe(true)
  })

  it("serve the current standard version as latest", async () => {
    const versioned = await readTree(VERSION)
    const latest = await readTree("latest")
    const pinned = new Map(
      [...latest].map(([name, text]) => [
        name,
        text.replaceAll("/profiles/latest/", `/profiles/${VERSION}/`),
      ]),
    )
    expect(pinned).toEqual(versioned)
  })
})

async function readTree(tag: string) {
  const folder = join(PUBLISHED, tag)
  const names = await readdir(folder, { recursive: true })
  const tree = new Map<string, string>()
  for (const name of names.filter(name => name.endsWith(".json")).sort()) {
    tree.set(name, await readFile(join(folder, name), "utf8"))
  }
  return tree
}
