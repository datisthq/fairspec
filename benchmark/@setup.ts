import { execFileSync } from "node:child_process"
import { existsSync } from "node:fs"
import { chmod, mkdir, mkdtemp, rm, writeFile } from "node:fs/promises"
import { tmpdir } from "node:os"
import { dirname, join } from "node:path"
import { QSV_PATH, QSV_VERSION } from "./tools/qsv.ts"
import { RIVALS } from "./tools/rivals.ts"

execFileSync("uv", ["sync", "--frozen"], { cwd: RIVALS, stdio: "inherit" })

if (!existsSync(QSV_PATH)) {
  const url = `https://github.com/dathere/qsv/releases/download/${QSV_VERSION}/qsv-${QSV_VERSION}-x86_64-unknown-linux-gnu.zip`
  console.log(`Downloading ${url}`)
  const response = await fetch(url)
  if (!response.ok) throw new Error(`Download failed: ${response.status} ${url}`)

  const directory = await mkdtemp(join(tmpdir(), "fairspec-qsv-"))
  const archive = join(directory, "qsv.zip")
  await writeFile(archive, Buffer.from(await response.arrayBuffer()))
  await mkdir(dirname(QSV_PATH), { recursive: true })
  execFileSync("unzip", ["-o", "-j", archive, "qsv", "-d", dirname(QSV_PATH)], {
    stdio: "inherit",
  })
  await chmod(QSV_PATH, 0o755)
  await rm(directory, { recursive: true, force: true })
}

console.log(execFileSync(QSV_PATH, ["--version"], { encoding: "utf8" }).split(";")[0])
