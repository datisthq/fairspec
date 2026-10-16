import { execFileSync } from "node:child_process"
import { cp, readFile, readdir, rm, writeFile } from "node:fs/promises"
import { join } from "node:path"
import standardPackage from "../standard/package.json" with { type: "json" }
import {
  CHECKSUMS,
  PUBLISHED,
  ROOT,
  hashFolder,
  readChecksums,
} from "./-shared/profiles.ts"

const SOURCE = join(ROOT, "standard", "profiles")
const VERSION = standardPackage.version

const tags = execFileSync(
  "git",
  ["tag", "--list", `v${VERSION}`, `standard-v${VERSION}`],
  {
    cwd: ROOT,
    encoding: "utf8",
  },
)
if (tags.trim()) {
  throw new Error(
    `Standard ${VERSION} is already released (tag ${tags.trim()}); its published profiles never change`,
  )
}

for (const tag of [VERSION, "latest"]) {
  const folder = join(PUBLISHED, tag)
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

const checksums = await readChecksums()
checksums[VERSION] = await hashFolder(join(PUBLISHED, VERSION))
const versions = Object.keys(checksums).sort((a, b) =>
  a.localeCompare(b, "en", { numeric: true }),
)
const sorted = Object.fromEntries(versions.map(version => [version, checksums[version]]))
await writeFile(CHECKSUMS, `${JSON.stringify(sorted, null, 2)}\n`)
