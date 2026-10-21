import { readFile, readdir, rm, writeFile } from "node:fs/promises"
import { join } from "node:path"
import standardPackage from "../standard/package.json" with { type: "json" }
import {
  LIBRARIES,
  SOURCE,
  VERSION_SETTING,
  listBundledProfiles,
  resolveRefs,
} from "./-shared/profiles.ts"

const names = await listBundledProfiles()

for (const library of LIBRARIES) {
  for (const name of await readdir(library.profiles)) {
    if (name.endsWith(".json") && !names.includes(name)) {
      await rm(join(library.profiles, name))
    }
  }
  for (const name of names) {
    const text = await readFile(join(SOURCE, name), "utf8")
    await writeFile(join(library.profiles, name), resolveRefs(text, "latest"))
  }

  const settings = await readFile(library.settings, "utf8")
  const pinned = `$1"${standardPackage.version}"`
  await writeFile(library.settings, settings.replace(VERSION_SETTING, pinned))
}
