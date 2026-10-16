import { cp, readFile, readdir, rm, writeFile } from "node:fs/promises"
import { join } from "node:path"
import standardPackage from "../standard/package.json" with { type: "json" }
import {
  CHECKSUMS,
  PUBLISHED,
  SOURCE,
  hashFolder,
  readChecksums,
  resolveRefs,
} from "./-shared/profiles.ts"

const VERSION = standardPackage.version
const checksums = await readChecksums()

if (checksums[VERSION]) {
  console.log(`Standard ${VERSION} is already published; its profiles never change`)
  process.exit(0)
}

for (const tag of [VERSION, "latest"]) {
  const folder = join(PUBLISHED, tag)
  await rm(folder, { recursive: true, force: true })
  await cp(SOURCE, folder, { recursive: true })

  for (const entry of await readdir(folder, { recursive: true })) {
    if (!entry.endsWith(".json")) continue
    const path = join(folder, entry)
    await writeFile(path, resolveRefs(await readFile(path, "utf8"), tag))
  }
}

checksums[VERSION] = await hashFolder(join(PUBLISHED, VERSION))
const versions = Object.keys(checksums).sort((a, b) =>
  a.localeCompare(b, "en", { numeric: true }),
)
const sorted = Object.fromEntries(versions.map(version => [version, checksums[version]]))
await writeFile(CHECKSUMS, `${JSON.stringify(sorted, null, 2)}\n`)
