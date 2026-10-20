import { Link, createFileRoute } from "@tanstack/react-router"
import {
  ArrowRight,
  BookOpen,
  Bot,
  Braces,
  Database,
  ExternalLink,
  FileText,
  Sparkles,
  SquareTerminal,
  Table2,
} from "lucide-react"
import type { ComponentType, ReactNode, SVGProps } from "react"
import { useState } from "react"
import { BackToTop } from "livemark/components/BackToTop"
import { Banner } from "livemark/components/Banner"
import { Header } from "livemark/components/Header"
import { cn } from "cn"
import {
  Datist,
  FinalCta,
  Hero,
  LandingSection,
  Reveal,
  type Slide,
  actionClass,
  tk,
} from "../components/Landing.tsx"
import {
  BrokenCsv,
  InferPane,
  MembersCsv,
  PreviewPane,
  QueryPane,
  ValidatePane,
} from "../components/Panes.tsx"

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
    <>
      <Banner />
      <Header />
      <main className="flex flex-col">
        <Hero
          eyebrow="Python and TypeScript"
          title="Fairspec CLI Tool"
          lede={
            <>
              Bring the <strong className="text-foreground">Fairspec</strong> data
              management framework to your shell. Validate datasets, infer schemas, and
              query tables — from scripts, CI pipelines, or an interactive terminal.
            </>
          }
          actions={
            <>
              <a href="#installation" className={actionClass("default")}>
                Install
                <ArrowRight className="size-4" />
              </a>
              <a
                href="https://github.com/datisthq/fairspec"
                target="_blank"
                rel="noopener noreferrer"
                className={actionClass("outline")}
              >
                View on GitHub
                <ExternalLink className="size-4" />
              </a>
            </>
          }
          slides={slides}
        />
        <Installation language={language} onLanguageChange={setLanguage} />
        <Usage language={language} />
        <Commands language={language} />
        <FinalCta
          title={
            <>
              Script your data. <span className="text-primary">Check it in CI.</span>
            </>
          }
          lede="Read the full command reference, or hand the same commands to your AI assistant."
          actions={
            <>
              <Link
                to="/$"
                params={{ _splat: `${language}/terminal` }}
                className={actionClass("default")}
              >
                <BookOpen className="size-4" />
                Read the CLI docs
              </Link>
              <Link to="/mcp-server/" className={actionClass("ghost")}>
                <Sparkles className="size-4" />
                Try the MCP server
              </Link>
              <Link
                to="/$"
                params={{ _splat: `${language}/agent-skills` }}
                className={actionClass("ghost")}
              >
                <Bot className="size-4" />
                Add agent skills
              </Link>
            </>
          }
        />
        <Datist />
      </main>
      <BackToTop />
    </>
  )
}

/* ─────────────────────────── Hero ─────────────────────────── */

const membersCsv = { eyebrow: "Input", file: "members.csv", code: <MembersCsv /> }

const slides: Slide[] = [
  { id: "infer", title: "Infer", pane: <InferPane />, source: membersCsv },
  {
    id: "validate",
    title: "Validate",
    pane: <ValidatePane />,
    source: { eyebrow: "Input", file: "broken.csv", code: <BrokenCsv /> },
  },
  { id: "preview", title: "Preview", pane: <PreviewPane />, source: membersCsv },
  { id: "query", title: "Query", pane: <QueryPane />, source: membersCsv },
]

/* ─────────────────────────── Installation ─────────────────────────── */

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
    <LandingSection
      id="installation"
      title="Installation"
      lede={
        <>
          Both implementations ship the same <code>fairspec</code> command. Pick the
          ecosystem you already use.
        </>
      }
    >
      <Reveal>
        <div className="mx-auto max-w-4xl">
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
          <div className="rounded-card border border-border bg-card overflow-hidden">
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
        </div>
      </Reveal>
    </LandingSection>
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
    <LandingSection
      tinted
      title="Usage"
      lede={
        <>
          Every command takes local paths or URLs and prints JSON with <code>--json</code>
          . Try one of these.
        </>
      }
    >
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-3">
        {examples.map((e, i) => (
          <Reveal key={e.command} delayMs={i * 40}>
            <div
              className={cn(
                "rounded-card border border-border bg-card p-4 flex flex-wrap items-start gap-3 transition-opacity",
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
    </LandingSection>
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
    <LandingSection
      title="Commands"
      lede="Commands are grouped by what they operate on: datasets, tables, JSON data, and files."
    >
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {commandGroups.map((g, i) => (
          <Reveal key={g.entity} delayMs={i * 40}>
            <CommandCard group={g} language={props.language} />
          </Reveal>
        ))}
      </div>
    </LandingSection>
  )
}

function CommandCard(props: { group: CommandGroup; language: Language }) {
  const { icon: Icon, entity, title, verbs, description } = props.group
  return (
    <Link
      to="/$"
      params={{ _splat: `${props.language}/terminal/${entity}` }}
      className="group block h-full relative rounded-card border border-border bg-card p-6 no-underline transition-colors hover:border-primary/40"
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
    </Link>
  )
}

function shellTokenClass(part: string, index: number) {
  if (index === 0) return tk.cmd
  if (part.startsWith('"')) return tk.str
  if (part.startsWith("-")) return tk.key
  if (part === ">") return tk.punct
  return undefined
}
