import { createHash } from "node:crypto"
import { existsSync } from "node:fs"
import { readFile, readdir } from "node:fs/promises"
import { join, resolve } from "node:path"

export const ROOT = resolve(import.meta.dirname, "..", "..")
export const PUBLISHED = join(ROOT, ".livemark", "public", "profiles")
export const CHECKSUMS = join(PUBLISHED, "checksums.json")

export type Checksums = Record<string, Record<string, string>>

export async function hashFolder(folder: string) {
  const names = await readdir(folder, { recursive: true, withFileTypes: true })
  const hashes: Record<string, string> = {}
  const files = names.filter(entry => entry.isFile())
  for (const path of files.map(entry => join(entry.parentPath, entry.name)).sort()) {
    const name = path.slice(folder.length + 1).replaceAll("\\", "/")
    hashes[name] = createHash("sha256")
      .update(await readFile(path))
      .digest("hex")
  }
  return hashes
}

export async function readChecksums() {
  const checksums: Checksums = existsSync(CHECKSUMS)
    ? JSON.parse(await readFile(CHECKSUMS, "utf8"))
    : {}
  return checksums
}
