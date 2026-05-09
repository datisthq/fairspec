import { execa } from "execa"
import { replaceInFile } from "replace-in-file"
import tasuku from "tasuku"
import packageJson from "./package.json" with { type: "json" }

process.chdir(import.meta.dirname)
const version = packageJson.version
const $ = execa({ stdout: ["inherit"], preferLocal: true })

await tasuku("Copying profiles", async () => {
  await $`rm -rf .livemark/public/profiles/latest`
  await $`rm -rf .livemark/public/profiles/${version}`
  await $`cp -r profiles .livemark/public/profiles/latest`
  await $`cp -r profiles .livemark/public/profiles/${version}`
})

await tasuku("Updating references", async () => {
  for (const tag of [packageJson.version, "latest"]) {
    for (const ext of ["datacite", "grei"]) {
      await replaceInFile({
        files: `.livemark/public/profiles/${tag}/${ext}/dataset.json`,
        from: /{dataset-ref}/g,
        to: `https://fairspec.org/profiles/${tag}/dataset.json`,
      })
    }

    await replaceInFile({
      files: `.livemark/public/profiles/${tag}/dataset.json`,
      from: /{file-dialect-ref}/g,
      to: `https://fairspec.org/profiles/${tag}/file-dialect.json`,
    })

    await replaceInFile({
      files: `.livemark/public/profiles/${tag}/dataset.json`,
      from: /{data-schema-ref}/g,
      to: `https://fairspec.org/profiles/${tag}/data-schema.json`,
    })

    await replaceInFile({
      files: `.livemark/public/profiles/${tag}/dataset.json`,
      from: /{table-schema-ref}/g,
      to: `https://fairspec.org/profiles/${tag}/table-schema.json`,
    })
  }
})
