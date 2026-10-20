import { createFileRoute } from "@tanstack/react-router"
import {
  ArrowRight,
  Braces,
  Database,
  ExternalLink,
  FileText,
  SquareTerminal,
  Table2,
} from "lucide-react"
import type { ComponentType, ReactNode, SVGProps } from "react"
import { useState } from "react"
import { buttonVariants } from "livemark/elements/button"
import { useInView } from "livemark/hooks/in-view"
import { cn } from "cn"

// @ts-ignore
export const Route = createFileRoute("/cli-tool/")({
  component: CliToolPage,
  head: () => ({
    meta: [
      { title: "Fairspec CLI Tool" },
      {
        name: "description",
        content: "Command-line interface for the Fairspec data management framework.",
      },
    ],
  }),
})

type Language = "python" | "typescript"

const languages: { key: Language; name: string }[] = [
  { key: "python", name: "Python" },
  { key: "typescript", name: "TypeScript" },
]

function CliToolPage() {
  const [language, setLanguage] = useState<Language>("python")
  return (
    <div className="flex flex-col">
      <Hero />
      <Installation language={language} onLanguageChange={setLanguage} />
      <Usage language={language} />
      <Commands language={language} />
    </div>
  )
}

/* ─────────────────────────── Hero ─────────────────────────── */

function Hero() {
  return (
    <section className="relative overflow-hidden border-b border-border flex items-center min-h-[calc(100vh-4rem)]">
      <BackgroundGrid />
      <div className="relative w-full mx-auto max-w-7xl px-6 py-16 md:py-24 animate-in fade-in-0 slide-in-from-bottom-4 duration-700 ease-out">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-16 items-center">
          <div className="text-center lg:text-left">
            <div className="inline-flex items-center gap-2 rounded-full border border-border bg-card px-3 py-1 text-xs text-muted-foreground mb-6">
              <SquareTerminal className="size-3.5 text-primary" />
              Python and TypeScript
            </div>
            <h1 className="text-5xl md:text-6xl font-extrabold tracking-tight text-foreground">
              Fairspec CLI Tool
            </h1>
            <p className="mt-6 text-lg md:text-xl text-muted-foreground">
              Bring the <strong className="text-foreground">Fairspec</strong> data
              management framework to your shell. Validate datasets, infer schemas, and
              query tables — from scripts, CI pipelines, or an interactive terminal.
            </p>
            <div className="mt-10 flex flex-col sm:flex-row items-center justify-center lg:justify-start gap-3">
              <a
                href="#installation"
                className={cn(
                  buttonVariants({ variant: "default", size: "lg" }),
                  "px-5 no-underline",
                )}
              >
                Install
                <ArrowRight className="size-4" />
              </a>
              <a
                href="https://github.com/datisthq/fairspec"
                target="_blank"
                rel="noopener noreferrer"
                className={cn(
                  buttonVariants({ variant: "outline", size: "lg" }),
                  "px-5 no-underline",
                )}
              >
                View on GitHub
                <ExternalLink className="size-4" />
              </a>
            </div>
          </div>
          <Transcript />
        </div>
      </div>
    </section>
  )
}

function Transcript() {
  return (
    <div className="rounded-xl border border-primary/20 bg-card overflow-hidden text-left">
      <div className="flex items-center gap-2 border-b border-primary/20 px-4 py-2 bg-muted/50">
        <div className="size-2.5 rounded-full bg-red-400/60" />
        <div className="size-2.5 rounded-full bg-yellow-400/60" />
        <div className="size-2.5 rounded-full bg-green-400/60" />
        <span className="ml-2 text-xs font-mono text-muted-foreground">Terminal</span>
      </div>
      <div className="p-5 space-y-4 font-mono text-sm leading-relaxed overflow-x-auto">
        <div className="flex items-start gap-2">
          <span className="text-primary shrink-0 select-none">$</span>
          <Shell command="fairspec table infer-schema members.csv --json" />
        </div>
        <pre className="rounded-md border border-border bg-background p-4 overflow-x-auto">
          <TableSchemaSample />
        </pre>
      </div>
    </div>
  )
}

function TableSchemaSample() {
  return (
    <code className={tk.body}>
      <span className={tk.punct}>{"{"}</span>
      {"\n  "}
      <span className={tk.key}>"properties"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{"}</span>
      {"\n    "}
      <span className={tk.key}>"id"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{ "}</span>
      <span className={tk.key}>"type"</span>
      <span className={tk.punct}>:</span> <span className={tk.str}>"integer"</span>
      <span className={tk.punct}>{" }"}</span>
      <span className={tk.punct}>,</span>
      {"\n    "}
      <span className={tk.key}>"name"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{ "}</span>
      <span className={tk.key}>"type"</span>
      <span className={tk.punct}>:</span> <span className={tk.str}>"string"</span>
      <span className={tk.punct}>{" }"}</span>
      <span className={tk.punct}>,</span>
      {"\n    "}
      <span className={tk.key}>"joined"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{ "}</span>
      <span className={tk.key}>"type"</span>
      <span className={tk.punct}>:</span> <span className={tk.str}>"string"</span>
      <span className={tk.punct}>,</span> <span className={tk.key}>"format"</span>
      <span className={tk.punct}>:</span> <span className={tk.str}>"date"</span>
      <span className={tk.punct}>{" }"}</span>
      <span className={tk.punct}>,</span>
      {"\n    "}
      <span className={tk.key}>"score"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{ "}</span>
      <span className={tk.key}>"type"</span>
      <span className={tk.punct}>:</span> <span className={tk.str}>"number"</span>
      <span className={tk.punct}>{" }"}</span>
      {"\n  "}
      <span className={tk.punct}>{"}"}</span>
      {"\n"}
      <span className={tk.punct}>{"}"}</span>
    </code>
  )
}

function BackgroundGrid() {
  return (
    <>
      <div
        aria-hidden
        className="absolute inset-0 [background-image:repeating-linear-gradient(90deg,var(--color-border)_0,var(--color-border)_1px,transparent_1px,transparent_8px)] opacity-25 [mask-image:linear-gradient(to_top,black_10%,transparent_85%)]"
      />
      <div
        aria-hidden
        className="absolute -bottom-40 -left-40 h-[36rem] w-[36rem] rounded-full bg-blue-400/30 dark:bg-blue-500/25 blur-[110px] pointer-events-none"
      />
      <div
        aria-hidden
        className="absolute -top-32 -right-32 h-[32rem] w-[32rem] rounded-full bg-sky-400/25 dark:bg-sky-500/20 blur-[110px] pointer-events-none"
      />
    </>
  )
}

/* ─────────────────────────── Installation ─────────────────────────── */

const tk = {
  punct: "text-[#7c7f93] dark:text-[#9399b2]",
  key: "text-[#1e66f5] dark:text-[#89b4fa]",
  str: "text-[#40a02b] dark:text-[#a6e3a1]",
  cmd: "text-[#8839ef] dark:text-[#cba6f7]",
  body: "text-[#4c4f69] dark:text-[#cdd6f4]",
}

function Shell({ command }: { command: string }) {
  const parts = command.match(/"[^"]*"|\s+|\S+/g) ?? []
  return (
    <code className={tk.body}>
      {parts.map((part, i) => (
        <span key={i} className={shellTokenClass(part, i)}>
          {part}
        </span>
      ))}
    </code>
  )
}

interface InstallMethod {
  name: string
  description: ReactNode
  command: string
}

const installs: Record<Language, InstallMethod[]> = {
  python: [
    {
      name: "uv",
      description: (
        <>
          Install the <code>fairspec</code> command into an isolated environment with{" "}
          <a href="https://docs.astral.sh/uv/" target="_blank" rel="noopener noreferrer">
            uv
          </a>
          .
        </>
      ),
      command: "uv tool install fairspec",
    },
    {
      name: "pipx",
      description: (
        <>
          Install the <code>fairspec</code> command into an isolated environment with
          pipx.
        </>
      ),
      command: "pipx install fairspec",
    },
    {
      name: "pip",
      description: <>Install into the active Python environment, library included.</>,
      command: "pip install fairspec",
    },
    {
      name: "uvx",
      description: <>Run the latest version without installing anything.</>,
      command: "uvx fairspec --help",
    },
  ],
  typescript: [
    {
      name: "npm",
      description: (
        <>
          Install the <code>fairspec</code> command globally with npm.
        </>
      ),
      command: "npm install -g fairspec",
    },
    {
      name: "pnpm",
      description: (
        <>
          Install the <code>fairspec</code> command globally with pnpm.
        </>
      ),
      command: "pnpm add -g fairspec",
    },
    {
      name: "npx",
      description: <>Run the latest version without installing anything.</>,
      command: "npx fairspec@latest --help",
    },
  ],
}

const terminalPackages: Record<Language, string> = {
  python: "fairspec-terminal",
  typescript: "@fairspec/terminal",
}

function Installation(props: {
  language: Language
  onLanguageChange: (language: Language) => void
}) {
  const [active, setActive] = useState(0)
  const methods = installs[props.language]
  const method = methods[active] ?? methods[0]
  if (!method) return null
  return (
    <section id="installation" className="border-b border-border scroll-mt-16">
      <div className="mx-auto max-w-5xl px-6 py-24">
        <Reveal>
          <div className="text-center max-w-2xl mx-auto mb-12">
            <h2 className="text-3xl md:text-4xl font-bold tracking-tight">
              Installation
            </h2>
            <p className="mt-4 text-muted-foreground text-lg">
              Both implementations ship the same <code>fairspec</code> command. Pick the
              ecosystem you already use.
            </p>
          </div>
        </Reveal>
        <Reveal>
          <div className="flex justify-center mb-6">
            <div
              role="tablist"
              className="inline-flex rounded-lg border border-border bg-muted/40 p-1"
            >
              {languages.map(l => (
                <button
                  key={l.key}
                  type="button"
                  role="tab"
                  aria-selected={l.key === props.language}
                  onClick={() => {
                    props.onLanguageChange(l.key)
                    setActive(0)
                  }}
                  className={cn(
                    "px-4 py-1.5 text-sm font-medium rounded-md transition-colors",
                    l.key === props.language
                      ? "bg-background text-foreground shadow-sm"
                      : "text-muted-foreground hover:text-foreground",
                  )}
                >
                  {l.name}
                </button>
              ))}
            </div>
          </div>
          <div className="rounded-xl border border-border bg-card overflow-hidden">
            <div
              role="tablist"
              className="flex flex-wrap border-b border-border bg-muted/40"
            >
              {methods.map((m, i) => (
                <button
                  key={m.name}
                  type="button"
                  role="tab"
                  aria-selected={i === active}
                  onClick={() => setActive(i)}
                  className={cn(
                    "px-4 py-3 text-sm font-medium transition-colors border-b-2",
                    i === active
                      ? "border-primary text-foreground"
                      : "border-transparent text-muted-foreground hover:text-foreground",
                  )}
                >
                  {m.name}
                </button>
              ))}
            </div>
            <div className="p-6">
              <p className="text-sm text-muted-foreground mb-4">{method.description}</p>
              <pre className="rounded-md border border-border bg-background p-4 text-sm leading-relaxed font-mono overflow-x-auto">
                <Shell command={method.command} />
              </pre>
            </div>
          </div>
          <p className="mt-6 text-center text-sm text-muted-foreground">
            Need only the command line, without the library? Install{" "}
            <code>{terminalPackages[props.language]}</code> instead.
          </p>
        </Reveal>
      </div>
    </section>
  )
}

/* ─────────────────────────── Usage ─────────────────────────── */

interface Example {
  command: string
  only?: Language
}

const examples: Example[] = [
  { command: "fairspec dataset infer *.csv --json > dataset.json" },
  { command: "fairspec dataset validate dataset.json" },
  { command: "fairspec table describe data.csv" },
  { command: "fairspec table preview data.csv" },
  { command: 'fairspec table query data.csv "SELECT * FROM self WHERE age > 25"' },
  { command: "fairspec table validate data.csv --json" },
  { command: "fairspec file describe data.csv --hash-type sha256" },
  { command: "fairspec mcp", only: "typescript" },
]

function Usage(props: { language: Language }) {
  return (
    <section className="border-b border-border">
      <div className="mx-auto max-w-6xl px-6 py-24">
        <Reveal>
          <div className="text-center max-w-2xl mx-auto mb-16">
            <h2 className="text-3xl md:text-4xl font-bold tracking-tight">Usage</h2>
            <p className="mt-4 text-muted-foreground text-lg">
              Every command takes local paths or URLs and prints JSON with{" "}
              <code>--json</code>. Try one of these.
            </p>
          </div>
        </Reveal>
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-3">
          {examples.map((e, i) => (
            <Reveal key={e.command} delayMs={i * 40}>
              <div
                className={cn(
                  "rounded-xl border border-border bg-card p-4 flex items-start gap-3 transition-opacity",
                  e.only && e.only !== props.language && "opacity-50",
                )}
              >
                <SquareTerminal className="size-4 mt-0.5 text-primary shrink-0" />
                <pre className="flex-1 min-w-0 overflow-x-auto text-sm font-mono leading-relaxed">
                  <Shell command={e.command} />
                </pre>
                {e.only && (
                  <span className="shrink-0 rounded-full border border-border bg-muted/60 px-2 py-0.5 text-xs text-muted-foreground">
                    {e.only === "python" ? "Python" : "TypeScript"} only
                  </span>
                )}
              </div>
            </Reveal>
          ))}
        </div>
      </div>
    </section>
  )
}

/* ─────────────────────────── Commands ─────────────────────────── */

interface CommandGroup {
  icon: ComponentType<SVGProps<SVGSVGElement>>
  entity: "dataset" | "table" | "data" | "file"
  title: string
  verbs: string[]
  description: string
}

const commandGroups: CommandGroup[] = [
  {
    icon: Database,
    entity: "dataset",
    title: "Datasets",
    verbs: ["infer", "validate", "copy", "list", "query", "script"],
    description:
      "Infer a dataset descriptor from a set of files, validate it with every resource it references, list or copy its resources, join them in one SQL query, or open it in an interactive session.",
  },
  {
    icon: Table2,
    entity: "table",
    title: "Tables",
    verbs: [
      "describe",
      "preview",
      "query",
      "validate",
      "infer-schema",
      "validate-schema",
      "render-schema",
      "infer-dialect",
      "script",
    ],
    description:
      "Describe, preview, and query tabular files with SQL, validate them against a table schema, and infer schemas and dialects from the data itself.",
  },
  {
    icon: Braces,
    entity: "data",
    title: "JSON Data",
    verbs: ["validate", "infer-schema", "validate-schema"],
    description:
      "Validate JSON data against a JSON Schema, infer a schema from a sample, and check that a data schema is well-formed.",
  },
  {
    icon: FileText,
    entity: "file",
    title: "Files",
    verbs: ["describe", "preview", "validate", "copy"],
    description:
      "Compute file sizes and hashes, preview the raw source, check integrity against an expected hash, and copy local or remote files and dataset resources.",
  },
]

function Commands(props: { language: Language }) {
  return (
    <section className="border-b border-border">
      <div className="mx-auto max-w-6xl px-6 py-24">
        <Reveal>
          <div className="text-center max-w-2xl mx-auto mb-16">
            <h2 className="text-3xl md:text-4xl font-bold tracking-tight">Commands</h2>
            <p className="mt-4 text-muted-foreground text-lg">
              Commands are grouped by what they operate on: datasets, tables, JSON data,
              and files.
            </p>
          </div>
        </Reveal>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {commandGroups.map((g, i) => (
            <Reveal key={g.entity} delayMs={i * 40}>
              <CommandCard group={g} language={props.language} />
            </Reveal>
          ))}
        </div>
      </div>
    </section>
  )
}

function CommandCard(props: { group: CommandGroup; language: Language }) {
  const { icon: Icon, entity, title, verbs, description } = props.group
  return (
    <a
      href={`/${props.language}/terminal/${entity}/`}
      className="group block h-full relative rounded-xl border border-border bg-card p-6 no-underline transition-colors hover:border-primary/40"
    >
      <div className="flex items-center gap-3 mb-3">
        <div className="inline-flex items-center justify-center size-10 rounded-lg bg-primary/10 text-primary">
          <Icon className="size-5" />
        </div>
        <h3 className="text-lg font-semibold text-foreground">{title}</h3>
        <ArrowRight className="ml-auto size-4 text-muted-foreground transition-transform group-hover:translate-x-0.5" />
      </div>
      <p className="text-sm text-muted-foreground leading-relaxed">{description}</p>
      <div className="mt-4 flex flex-wrap gap-1.5">
        {verbs.map(v => (
          <code
            key={v}
            className="rounded-md border border-border bg-muted/60 px-2 py-0.5 text-xs text-foreground"
          >
            fairspec {entity} {v}
          </code>
        ))}
      </div>
    </a>
  )
}

/* ─────────────────────────── Reveal helper ─────────────────────────── */

function Reveal(props: { children: ReactNode; delayMs?: number }) {
  const { ref, isVisible } = useInView()
  return (
    <div
      ref={ref as (node: HTMLDivElement | null) => void}
      style={{ transitionDelay: `${props.delayMs ?? 0}ms` }}
      className={cn(
        "transition-all duration-700 ease-out",
        isVisible ? "opacity-100 translate-y-0" : "opacity-0 translate-y-4",
      )}
    >
      {props.children}
    </div>
  )
}

function shellTokenClass(part: string, index: number) {
  if (index === 0) return tk.cmd
  if (part.startsWith('"')) return tk.str
  if (part.startsWith("-")) return tk.key
  if (part === ">") return tk.punct
  return undefined
}
