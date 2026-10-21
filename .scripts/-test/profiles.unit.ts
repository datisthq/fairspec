import { existsSync } from "node:fs"
import { readFile, readdir } from "node:fs/promises"
import { join } from "node:path"
import { describe, expect, it } from "vite-plus/test"
import standardPackage from "../../standard/package.json" with { type: "json" }
import {
  LIBRARIES,
  PUBLISHED,
  SOURCE,
  VERSION_SETTING,
  hashFolder,
  listBundledProfiles,
  readChecksums,
  resolveRefs,
} from "../-shared/profiles.ts"

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

  it("match their recorded checksums", async () => {
    const checksums = await readChecksums()
    const entries = await readdir(PUBLISHED, { withFileTypes: true })
    const versions = entries
      .filter(entry => entry.isDirectory() && entry.name !== "latest")
      .map(entry => entry.name)
    expect(Object.keys(checksums).sort()).toEqual(versions.sort())
    for (const version of versions) {
      expect(await hashFolder(join(PUBLISHED, version)), version).toEqual(
        checksums[version],
      )
    }
  })
})

describe.each(LIBRARIES)("bundled profiles in $profiles", library => {
  it("equal the standard profiles", async () => {
    const names = await listBundledProfiles()
    const bundled = (await readdir(library.profiles)).filter(name =>
      name.endsWith(".json"),
    )
    expect(bundled.sort()).toEqual(names)
    for (const name of names) {
      const source = await readFile(join(SOURCE, name), "utf8")
      const copy = await readFile(join(library.profiles, name), "utf8")
      expect(JSON.parse(copy), name).toEqual(JSON.parse(resolveRefs(source, "latest")))
    }
  })

  it("pin the current standard version", async () => {
    const settings = await readFile(library.settings, "utf8")
    expect(VERSION_SETTING.exec(settings)?.[2]).toBe(VERSION)
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
