import { Link, createFileRoute } from "@tanstack/react-router"
import {
  ArrowRight,
  BookOpen,
  Bot,
  Braces,
  Database,
  ExternalLink,
  FileText,
  Globe,
  ListChecks,
  Package,
  PlayCircle,
  Sliders,
  Sparkles,
  SquareTerminal,
  Table2,
} from "lucide-react"
import type { ReactNode } from "react"
import { useState } from "react"
import { BackToTop } from "livemark/components/BackToTop"
import { Banner } from "livemark/components/Banner"
import { Header } from "livemark/components/Header"
import { cn } from "cn"
import {
  Datist,
  FeatureGrid,
  FinalCta,
  Hero,
  type Language,
  LandingSection,
  LanguageTabs,
  type Point,
  ShowcaseSection,
  type Slide,
  SplitSection,
  WindowCard,
  actionClass,
  languages,
  textLinkClass,
  tk,
} from "../components/Landing.tsx"
import {
  BrokenCsv,
  MembersCsv,
  HeroTerminal,
  InferSession,
  PreviewSession,
  QuerySession,
  ValidateSession,
  ValidationErrors,
  WindowCode,
} from "../components/Panes.tsx"

// @ts-ignore
export const Route = createFileRoute("/cli-tool/")({
  component: CliToolPage,
  head: () => ({
    meta: [
      { title: "Fairspec CLI" },
      {
        name: "description",
        content: "Command-line interface for the Fairspec data management framework.",
      },
    ],
  }),
})

function CliToolPage() {
  const [language, setLanguage] = useState<Language>("python")
  const docs = readDocsSection(language)
  return (
    <>
      <Banner />
      <Header />
      <main className="flex flex-col">
        <Hero
          eyebrow="Open source · MIT License"
          title="Fairspec CLI"
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
        <SplitSection
          id="installation"
          title="One command, three runtimes"
          lede={
            <>
              Every implementation ships the same <code>fairspec</code> command. Pick the
              ecosystem you already use.
            </>
          }
          switcher={<LanguageTabs language={language} onChange={setLanguage} />}
          points={installPoints}
          visual={<InstallWindow key={language} language={language} />}
        />
        <LandingSection
          title="Commands"
          lede="Commands are grouped by what they operate on: datasets, tables, JSON data, and files."
        >
          <FeatureGrid columns={2} features={createCommandPoints(docs)} />
        </LandingSection>
        <ShowcaseSection
          title="Same command, both runtimes"
          lede="The Python and TypeScript CLIs take the same arguments and print the same report."
          left={{ label: "Python", content: <ValidateTranscript /> }}
          right={{ label: "TypeScript", content: <ValidateTranscript /> }}
        />
        <SplitSection
          reverse
          title="Built for scripts and CI"
          lede="Every command takes local paths or URLs and prints JSON with --json."
          points={scriptPoints}
          visual={<ExamplesWindow language={language} />}
          links={
            <Link
              to="/$"
              params={{ _splat: `${docs}/terminal` }}
              className={textLinkClass}
            >
              Read the CLI docs →
            </Link>
          }
        />
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
                params={{ _splat: `${docs}/terminal` }}
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
                params={{ _splat: `${docs}/agent-skills` }}
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
  {
    id: "infer",
    title: "Infer",
    content: (
      <HeroTerminal>
        <InferSession />
      </HeroTerminal>
    ),
    source: membersCsv,
  },
  {
    id: "validate",
    title: "Validate",
    content: (
      <HeroTerminal>
        <ValidateSession />
      </HeroTerminal>
    ),
    source: { eyebrow: "Input", file: "broken.csv", code: <BrokenCsv /> },
  },
  {
    id: "preview",
    title: "Preview",
    content: (
      <HeroTerminal>
        <PreviewSession />
      </HeroTerminal>
    ),
    source: membersCsv,
  },
  {
    id: "query",
    title: "Query",
    content: (
      <HeroTerminal>
        <QuerySession />
      </HeroTerminal>
    ),
    source: membersCsv,
  },
]

/* ─────────────────────────── Installation ─────────────────────────── */

function Shell(props: { command: string }) {
  const parts = props.command.match(/"[^"]*"|\s+|\S+/g) ?? []
  return (
    <>
      {parts.map((part, i) => (
        <span key={i} className={shellTokenClass(part, i)}>
          {part}
        </span>
      ))}
    </>
  )
}

interface InstallMethod {
  name: string
  description: ReactNode
  command: string
}

const RUST_RELEASES = "https://github.com/datisthq/fairspec/releases?q=rust-v"

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
  rust: [
    {
      name: "script",
      description: (
        <>
          Download the <code>fairspec</code> binary for your platform from the{" "}
          <a href={RUST_RELEASES} target="_blank" rel="noopener noreferrer">
            GitHub releases
          </a>
          , check its checksum, and put it in <code>~/.local/bin</code>.
        </>
      ),
      command: "curl -fsSL https://fairspec.org/install.sh | sh -s -- terminal",
    },
    {
      name: "cargo",
      description: (
        <>
          Build the <code>fairspec</code> command from source with cargo. The crates are
          not on crates.io yet.
        </>
      ),
      command:
        "cargo install --locked --git https://github.com/datisthq/fairspec fairspec",
    },
  ],
}

const terminalPackages: Partial<Record<Language, string>> = {
  python: "fairspec-terminal",
  typescript: "@fairspec/terminal",
}

const installPoints: Point[] = [
  {
    icon: SquareTerminal,
    title: "The same fairspec command",
    description:
      "Python, TypeScript and Rust install one command with the same subcommands and options.",
  },
  {
    icon: Package,
    title: "Library or command line only",
    description: (
      <>
        <code className="font-mono text-foreground">fairspec</code> brings the library
        too; <code className="font-mono text-foreground">fairspec-terminal</code> and{" "}
        <code className="font-mono text-foreground">@fairspec/terminal</code> ship just
        the CLI.
      </>
    ),
  },
  {
    icon: PlayCircle,
    title: "Run without installing",
    description: (
      <>
        <code className="font-mono text-foreground">uvx fairspec</code> or{" "}
        <code className="font-mono text-foreground">npx fairspec@latest</code> runs the
        latest version on demand.
      </>
    ),
  },
  {
    icon: Sparkles,
    title: "MCP server included",
    description: (
      <>
        <code className="font-mono text-foreground">fairspec mcp</code> starts the MCP
        server from the same install.
      </>
    ),
  },
]

function InstallWindow(props: { language: Language }) {
  const [active, setActive] = useState(0)
  const methods = installs[props.language]
  const method = methods[active] ?? methods[0]
  const terminalPackage = terminalPackages[props.language]
  if (!method) return null
  return (
    <WindowCard label="Install">
      <div role="tablist" className="flex flex-wrap border-b border-border bg-muted/40">
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
          <code className={tk.body}>
            <Shell command={method.command} />
          </code>
        </pre>
        {terminalPackage ? (
          <p className="mt-4 text-sm text-muted-foreground">
            Need only the command line, without the library? Install{" "}
            <code>{terminalPackage}</code> instead.
          </p>
        ) : null}
      </div>
    </WindowCard>
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
  { command: 'fairspec table validate data.csv --check "positive-price=price > 0"' },
  { command: "fairspec file describe data.csv --hash-type sha256" },
  { command: "fairspec mcp" },
]

const scriptPoints: Point[] = [
  {
    icon: Globe,
    title: "Local paths or URLs",
    description:
      "Point any table, file or dataset command at a file on disk or a remote URL.",
  },
  {
    icon: Braces,
    title: "JSON for pipelines",
    description: (
      <>
        Add <code className="font-mono text-foreground">--json</code> and the result is
        data your script or CI job can read, not prose.
      </>
    ),
  },
  {
    icon: ListChecks,
    title: "Custom checks",
    description: (
      <>
        Add rules the schema cannot express with{" "}
        <code className="font-mono text-foreground">--check name=condition</code>: a SQL
        condition every row must meet, or an aggregate over the whole table.
      </>
    ),
  },
  {
    icon: Sliders,
    title: "Dialects on the command line",
    description: (
      <>
        Override what inference guesses with{" "}
        <code className="font-mono text-foreground">--delimiter</code>,{" "}
        <code className="font-mono text-foreground">--header-rows</code> or{" "}
        <code className="font-mono text-foreground">--sheet-name</code>.
      </>
    ),
  },
  {
    icon: PlayCircle,
    title: "Interactive sessions",
    description: (
      <>
        <code className="font-mono text-foreground">fairspec table script</code> and{" "}
        <code className="font-mono text-foreground">fairspec dataset script</code> open a
        scripting session on the loaded data.
      </>
    ),
  },
]

function ExamplesWindow(props: { language: Language }) {
  return (
    <WindowCard label="examples">
      <ul className="divide-y divide-border">
        {examples.map(e => (
          <li
            key={e.command}
            className={cn(
              "flex flex-wrap items-start gap-3 px-5 py-3 transition-opacity",
              e.only && e.only !== props.language && "opacity-50",
            )}
          >
            <pre className="flex-1 min-w-0 overflow-x-auto text-sm font-mono leading-relaxed">
              <code className={tk.body}>
                <span className="text-primary select-none">$ </span>
                <Shell command={e.command} />
              </code>
            </pre>
            {e.only && (
              <span className="shrink-0 rounded-full border border-border bg-muted/60 px-2 py-0.5 text-xs text-muted-foreground">
                {languages.find(l => l.key === e.only)?.name} only
              </span>
            )}
          </li>
        ))}
      </ul>
    </WindowCard>
  )
}

function ValidateTranscript() {
  return (
    <WindowCode>
      <span className="text-primary select-none">$ </span>
      <Shell command="fairspec table validate broken.csv --schema schema.json" />
      {"\n\n"}
      <ValidationErrors />
    </WindowCode>
  )
}

/* ─────────────────────────── Commands ─────────────────────────── */

interface CommandGroup {
  icon: Point["icon"]
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

function createCommandPoints(docs: DocsSection) {
  return commandGroups.map(g => ({
    icon: g.icon,
    title: g.title,
    description: (
      <>
        <p>{g.description}</p>
        <div className="mt-4 flex flex-wrap gap-1.5">
          {g.verbs.map(v => (
            <code
              key={v}
              className="rounded-md border border-border bg-muted/60 px-2 py-0.5 text-xs text-foreground"
            >
              fairspec {g.entity} {v}
            </code>
          ))}
        </div>
      </>
    ),
    link: { to: "/$", splat: `${docs}/terminal/${g.entity}` },
  }))
}

function shellTokenClass(part: string, index: number) {
  if (index === 0) return tk.cmd
  if (part.startsWith('"')) return tk.str
  if (part.startsWith("-")) return tk.key
  if (part === ">") return tk.punct
  return undefined
}

type DocsSection = Exclude<Language, "rust">

// Rust has no docs section; its CLI takes the same commands, so it links to Python's.
function readDocsSection(language: Language): DocsSection {
  return language === "rust" ? "python" : language
}
