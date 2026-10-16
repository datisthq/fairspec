import { cp, readFile, readdir, rm, writeFile } from "node:fs/promises"
import { join, resolve } from "node:path"
import standardPackage from "../standard/package.json" with { type: "json" }

const ROOT = resolve(import.meta.dirname, "..")
const SOURCE = join(ROOT, "standard", "profiles")
const TARGET = join(ROOT, ".livemark", "public", "profiles")

for (const tag of [standardPackage.version, "latest"]) {
  const folder = join(TARGET, tag)
  await rm(folder, { recursive: true, force: true })
  await cp(SOURCE, folder, { recursive: true })

  for (const entry of await readdir(folder, { recursive: true })) {
    if (!entry.endsWith(".json")) continue
    const path = join(folder, entry)
    const text = await readFile(path, "utf8")
    const url = `https://fairspec.org/profiles/${tag}/$1.json`
    await writeFile(path, text.replace(/\{([a-z-]+)-ref\}/g, url))
  }
}
