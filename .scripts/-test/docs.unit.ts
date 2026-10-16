import { existsSync } from "node:fs"
import { readFile, readdir, readlink } from "node:fs/promises"
import { dirname, join, relative, resolve } from "node:path"
import { describe, expect, it } from "vite-plus/test"

const ROOT = resolve(import.meta.dirname, "..", "..")
const SKILLS = join(ROOT, ".agents", "skills")
const SECTIONS = ["catalog", "desktop", "python", "standard", "typescript"]

const ROOTED = [
  ...SECTIONS,
  ".agents",
  ".claude",
  ".github",
  ".humans",
  ".livemark",
  ".release",
  ".scripts",
]

const UNWALKED = [
  "node_modules",
  "build",
  "compile",
  "coverage",
  "htmlcov",
  "__pycache__",
]
const BUILTIN_COMMANDS = [
  "install",
  "exec",
  "add",
  "remove",
  "dlx",
  "why",
  "publish",
  "run",
]

describe("docs", () => {
  it.skipIf(process.platform === "win32")(
    "exposes skills to Claude Code as a symlink",
    async () => {
      const path = join(ROOT, ".claude", "skills")
      const target = await readlink(path)
      expect(resolve(dirname(path), target)).toBe(SKILLS)
    },
  )

  it("keeps exactly one AGENTS.md, at the root", async () => {
    const paths = await findFiles(ROOT, "AGENTS.md")
    expect(paths.map(path => relative(ROOT, path))).toEqual(["AGENTS.md"])
  })

  it("names only paths that exist", async () => {
    const missing: string[] = []
    for (const doc of await readDocs()) {
      for (const [, span = ""] of doc.text.matchAll(/`([^`\s]+)`/g)) {
        const path = await resolveRepoPath(span, doc.section)
        if (path && !existsSync(path)) missing.push(`${doc.path}: ${span}`)
      }
    }
    expect(missing).toEqual([])
  })

  it("names only pnpm scripts that exist", async () => {
    const scripts = new Map<string, string[]>()
    for (const base of ["", ...SECTIONS]) {
      scripts.set(base, await readScripts(base))
    }

    const missing: string[] = []
    const pattern = new RegExp(
      `pnpm (?:-C (${SECTIONS.join("|")}) )?([a-z][\\w:-]*)`,
      "g",
    )
    for (const doc of await readDocs()) {
      for (const [command, section = "", script = ""] of readCode(doc.text).matchAll(
        pattern,
      )) {
        if (BUILTIN_COMMANDS.includes(script)) continue
        if (!scripts.get(section)?.includes(script))
          missing.push(`${doc.path}: ${command}`)
      }
    }
    expect(missing).toEqual([])
  })

  it("gives every skill a name, a description, and provenance when vendored", async () => {
    const problems: string[] = []
    for (const name of await readdir(SKILLS)) {
      const path = join(SKILLS, name, "SKILL.md")
      if (!existsSync(path)) {
        problems.push(`${name}: no SKILL.md`)
        continue
      }

      const [, frontmatter = ""] = (await readFile(path, "utf-8")).split("---", 2)
      const description = /^description: (.*)$/m.exec(frontmatter)?.[1] ?? ""
      if (!new RegExp(`^name: ${name}$`, "m").test(frontmatter)) {
        problems.push(`${name}: frontmatter name must match the directory`)
      }
      if (!description.trim()) {
        problems.push(`${name}: no description, which is what decides the skill is used`)
      }
      if (description.length > 250) {
        problems.push(
          `${name}: description is ${description.length} chars, keep it ≤ 250`,
        )
      }
      if (/^source:/m.test(frontmatter) !== /^synced:/m.test(frontmatter)) {
        problems.push(`${name}: a vendored skill needs both source and synced`)
      }
    }
    expect(problems).toEqual([])
  })

  it("lists every skill in the AGENTS.md table, and only those", async () => {
    const agents = await readFile(join(ROOT, "AGENTS.md"), "utf-8")
    const listed = [...agents.matchAll(/^\| `([a-z-]+)` +\|/gm)].map(match => match[1])
    expect(listed.toSorted()).toEqual((await readdir(SKILLS)).toSorted())
  })
})

async function readDocs() {
  const paths = [
    "AGENTS.md",
    "README.md",
    "CONTRIBUTING.md",
    ...SECTIONS.flatMap(section => [
      `${section}/README.md`,
      `${section}/CONTRIBUTING.md`,
    ]),
    ...(await readdir(SKILLS)).map(name => `.agents/skills/${name}/SKILL.md`),
    ...(await findFiles(join(ROOT, ".humans"), ".md")).map(path => relative(ROOT, path)),
  ]

  const docs: { path: string; section?: string; text: string }[] = []
  for (const path of paths) {
    if (!existsSync(join(ROOT, path))) continue
    const text = await readFile(join(ROOT, path), "utf-8")
    docs.push({ path, section: findSection(path), text })
  }
  return docs
}

function findSection(path: string) {
  const [first] = path.split("/")
  if (first && SECTIONS.includes(first)) return first
  return /^\.agents\/skills\/([a-z]+)-development\//.exec(path)?.[1]
}

async function resolveRepoPath(span: string, section?: string) {
  if (/[*<>{}]|:\/\//.test(span) || span.startsWith("@")) return undefined

  const segments = span.replace(/\/$/, "").split("/")
  const [first] = segments
  if (!first || segments.length < 2 || segments.includes("build")) return undefined

  if (ROOTED.includes(first)) return join(ROOT, span)
  if (section && (await getIsDirectory(join(ROOT, section, first)))) {
    return join(ROOT, section, span)
  }
  return undefined
}

async function readScripts(base: string) {
  const path = join(ROOT, base, "package.json")
  if (!existsSync(path)) return []
  return Object.keys(JSON.parse(await readFile(path, "utf-8")).scripts ?? {})
}

function readCode(text: string) {
  const fences = [...text.matchAll(/```[\w]*\r?\n([\s\S]*?)```/g)].map(match => match[1])
  const spans = [...text.matchAll(/`([^`\n]+)`/g)].map(match => match[1])
  return [...fences, ...spans].join("\n")
}

async function findFiles(directory: string, suffix: string): Promise<string[]> {
  if (!existsSync(directory)) return []

  const paths: string[] = []
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name)
    if (entry.isDirectory()) {
      if (entry.name.startsWith(".") || UNWALKED.includes(entry.name)) continue
      paths.push(...(await findFiles(path, suffix)))
    } else if (entry.name.endsWith(suffix)) {
      paths.push(path)
    }
  }
  return paths
}

async function getIsDirectory(path: string) {
  try {
    return (await readdir(path, { withFileTypes: true })) !== undefined
  } catch {
    return false
  }
}
