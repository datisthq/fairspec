import { createHash } from "node:crypto"
import { existsSync } from "node:fs"
import { readFile, readdir } from "node:fs/promises"
import { join, resolve } from "node:path"

export const ROOT = resolve(import.meta.dirname, "..", "..")
export const SOURCE = join(ROOT, "standard", "profiles")
export const PUBLISHED = join(ROOT, ".livemark", "public", "profiles")
export const CHECKSUMS = join(PUBLISHED, "checksums.json")

export const LIBRARIES = [
  {
    profiles: join(ROOT, "typescript", "metadata", "profiles"),
    settings: join(ROOT, "typescript", "metadata", "settings.ts"),
  },
  {
    profiles: join(ROOT, "python", "metadata", "fairspec_metadata", "profiles"),
    settings: join(ROOT, "python", "metadata", "fairspec_metadata", "settings.py"),
  },
  {
    profiles: join(ROOT, "rust", "metadata", "profiles"),
    settings: join(ROOT, "rust", "metadata", "src", "settings.rs"),
  },
]

export const VERSION_SETTING = /(FAIRSPEC_VERSION(?:: &str)? = )"([^"]*)"/

export type Checksums = Record<string, Record<string, string>>

export function resolveRefs(text: string, tag: string) {
  return text.replace(
    /\{([a-z-]+)-ref\}/g,
    `https://fairspec.org/profiles/${tag}/$1.json`,
  )
}

export async function listBundledProfiles() {
  const entries = await readdir(SOURCE, { withFileTypes: true })
  return entries
    .filter(entry => entry.isFile() && entry.name.endsWith(".json"))
    .map(entry => entry.name)
    .sort()
}

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
