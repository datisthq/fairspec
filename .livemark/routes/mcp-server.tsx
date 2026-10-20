import { Link, createFileRoute } from "@tanstack/react-router"
import {
  Bot,
  CheckCircle2,
  ChevronRight,
  Eye,
  Github,
  ListOrdered,
  Rocket,
  Search,
  Sparkles,
  SquareTerminal,
  Wrench,
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
  LandingSection,
  type Point,
  Reveal,
  WindowCard,
  actionClass,
  tk,
} from "../components/Landing.tsx"

// @ts-ignore
export const Route = createFileRoute("/mcp-server/")({
  component: McpServerPage,
  head: () => ({
    meta: [
      { title: "Fairspec MCP Server" },
      {
        name: "description",
        content: "AI assistant integration for the Fairspec data management framework.",
      },
    ],
  }),
})

function McpServerPage() {
  return (
    <>
      <Banner />
      <Header />
      <main className="flex flex-col">
        <Hero
          eyebrow="Technical preview"
          title="Fairspec MCP Server"
          lede={
            <>
              Connect AI assistants to the{" "}
              <strong className="text-foreground">Fairspec</strong> data management
              framework. Validate datasets, infer schemas, and query tables — directly
              from your editor or chat.
            </>
          }
          actions={
            <>
              <a href="#installation" className={actionClass("default")}>
                <Rocket className="size-4" />
                Install
              </a>
              <a
                href="https://github.com/datisthq/fairspec"
                target="_blank"
                rel="noopener noreferrer"
                className={actionClass("outline")}
              >
                <Github className="size-4" />
                View source
              </a>
            </>
          }
          visual={<Transcript />}
        />
        <Installation />
        <Usage />
        <Tools />
        <FinalCta
          title={
            <>
              Ask in plain language.{" "}
              <span className="text-primary">Get FAIR data back.</span>
            </>
          }
          lede="Pair the server with the agent skills so your coding agent knows when to reach for each tool, or script the same operations from the CLI."
          actions={
            <>
              <Link to="/agent-skills/" className={actionClass("default")}>
                <Bot className="size-4" />
                Add agent skills
              </Link>
              <Link to="/cli-tool/" className={actionClass("ghost")}>
                <SquareTerminal className="size-4" />
                Use the CLI tool
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

function Transcript() {
  return (
    <WindowCard label="AI Assistant">
      <div className="p-5 space-y-4 font-mono text-sm leading-relaxed overflow-x-auto">
        <div className="flex items-start gap-2">
          <ChevronRight className="size-4 mt-0.5 text-primary shrink-0" />
          <span className="min-w-0 text-foreground">
            Infer the schema for members.csv
          </span>
        </div>
        <div className="flex items-center gap-2 text-xs">
          <span className="inline-flex items-center gap-1.5 rounded-md border border-border bg-muted/60 px-2 py-1 text-muted-foreground">
            <Wrench className="size-3 text-primary" />
            inferTableSchemaTool
          </span>
        </div>
        <pre className="rounded-md border border-border bg-background p-4 overflow-x-auto">
          <TableSchemaSample />
        </pre>
      </div>
    </WindowCard>
  )
}

/* ─────────────────────────── Installation ─────────────────────────── */

type Runtime = "node" | "python"

const runtimes: { id: Runtime; label: string }[] = [
  { id: "node", label: "Node.js" },
  { id: "python", label: "Python" },
]

const runners: Record<Runtime, string> = { node: "npx", python: "uvx" }

function BashAgentAdd({
  agent,
  runtime,
}: {
  agent: "claude" | "codex"
  runtime: Runtime
}) {
  return (
    <code className={tk.body}>
      <span className={tk.cmd}>{agent}</span> mcp add{" "}
      <span className={tk.str}>fairspec</span> <span className={tk.punct}>--</span>{" "}
      {runners[runtime]} <span className={tk.str}>fairspec@latest</span> mcp
    </code>
  )
}

function McpServersJson({
  rootKey,
  runtime,
}: {
  rootKey: "mcpServers" | "servers"
  runtime: Runtime
}) {
  return (
    <code className={tk.body}>
      <span className={tk.punct}>{"{"}</span>
      {"\n  "}
      <span className={tk.key}>"{rootKey}"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{"}</span>
      {"\n    "}
      <span className={tk.key}>"fairspec"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{"}</span>
      {"\n      "}
      <span className={tk.key}>"command"</span>
      <span className={tk.punct}>:</span>{" "}
      <span className={tk.str}>"{runners[runtime]}"</span>
      <span className={tk.punct}>,</span>
      {"\n      "}
      <span className={tk.key}>"args"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>[</span>
      <span className={tk.str}>"fairspec@latest"</span>
      <span className={tk.punct}>,</span> <span className={tk.str}>"mcp"</span>
      <span className={tk.punct}>]</span>
      {"\n    "}
      <span className={tk.punct}>{"}"}</span>
      {"\n  "}
      <span className={tk.punct}>{"}"}</span>
      {"\n"}
      <span className={tk.punct}>{"}"}</span>
    </code>
  )
}

interface Assistant {
  name: string
  description: ReactNode
  snippet: (runtime: Runtime) => ReactNode
}

const assistants: Assistant[] = [
  {
    name: "Claude Desktop",
    description: (
      <>
        Add to your <code>claude_desktop_config.json</code>:
      </>
    ),
    snippet: runtime => <McpServersJson rootKey="mcpServers" runtime={runtime} />,
  },
  {
    name: "Claude Code",
    description: <>Add the server with one command in any project directory.</>,
    snippet: runtime => <BashAgentAdd agent="claude" runtime={runtime} />,
  },
  {
    name: "Codex",
    description: (
      <>
        One command configures the Codex CLI, the ChatGPT desktop app, and the IDE
        extension — they share the same MCP configuration.
      </>
    ),
    snippet: runtime => <BashAgentAdd agent="codex" runtime={runtime} />,
  },
  {
    name: "Gemini CLI",
    description: (
      <>
        Add to <code>~/.gemini/settings.json</code>:
      </>
    ),
    snippet: runtime => <McpServersJson rootKey="mcpServers" runtime={runtime} />,
  },
]

function Installation() {
  const [active, setActive] = useState(0)
  const [runtime, setRuntime] = useState<Runtime>("node")
  const assistant = assistants[active] ?? assistants[0]
  if (!assistant) return null
  return (
    <LandingSection
      id="installation"
      tinted
      title="Installation"
      lede="Wire up the Fairspec MCP server in your favorite AI assistant. It ships with both the TypeScript and the Python package — pick the runtime you have."
    >
      <Reveal>
        <div className="max-w-4xl mx-auto rounded-card border border-border bg-card overflow-hidden">
          <div className="flex flex-wrap items-center justify-between gap-x-4 border-b border-border bg-muted/40">
            <div role="tablist" className="flex flex-wrap">
              {assistants.map((a, i) => (
                <button
                  key={a.name}
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
                  {a.name}
                </button>
              ))}
            </div>
            <div
              role="radiogroup"
              aria-label="Runtime"
              className="mx-4 my-2 inline-flex rounded-lg border border-border bg-background p-0.5"
            >
              {runtimes.map(r => (
                <button
                  key={r.id}
                  type="button"
                  role="radio"
                  aria-checked={r.id === runtime}
                  onClick={() => setRuntime(r.id)}
                  className={cn(
                    "rounded-md px-3 py-1 text-xs font-medium transition-colors",
                    r.id === runtime
                      ? "bg-primary text-primary-foreground"
                      : "text-muted-foreground hover:text-foreground",
                  )}
                >
                  {r.label}
                </button>
              ))}
            </div>
          </div>
          <div className="min-w-0 p-6">
            <p className="text-sm text-muted-foreground mb-4">{assistant.description}</p>
            <pre className="rounded-md border border-border bg-background p-4 text-sm leading-relaxed font-mono overflow-x-auto">
              {assistant.snippet(runtime)}
            </pre>
          </div>
        </div>
      </Reveal>
    </LandingSection>
  )
}

/* ─────────────────────────── Usage ─────────────────────────── */

const prompts: string[] = [
  "Validate my dataset in the current directory",
  "Infer the schema for data.csv",
  "Check if my data schema is valid",
  "Detect the dialect of measurements.tsv",
  "Query users.csv for all records where age > 30",
  "Infer a dataset descriptor from all files in the data folder",
  "Validate the table schema in schema.json",
  "Show me the first 10 rows of results.csv sorted by date",
]

function Usage() {
  return (
    <LandingSection
      title="Usage"
      lede="Just ask in natural language. Try one of these prompts."
    >
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
        {prompts.map((p, i) => (
          <Reveal key={p} delayMs={i * 40}>
            <div className="h-full rounded-card border border-border bg-card p-4 flex items-start gap-3">
              <Sparkles className="size-4 mt-0.5 text-primary shrink-0" />
              <p className="min-w-0 text-sm text-foreground leading-relaxed">{p}</p>
            </div>
          </Reveal>
        ))}
      </div>
    </LandingSection>
  )
}

/* ─────────────────────────── Tools ─────────────────────────── */

const tools: Point[] = [
  {
    icon: CheckCircle2,
    title: "Validate Data",
    description:
      "Validates data against a provided data schema, checking that every value conforms to the declared types and constraints. Returns detailed error reports pinpointing exactly which values failed validation and why.",
  },
  {
    icon: Search,
    title: "Infer Data Schema",
    description:
      "Analyzes a data sample and infers a data schema descriptor describing its structure, types, and constraints. Examines patterns across values to determine the most appropriate type for each field.",
  },
  {
    icon: CheckCircle2,
    title: "Validate Data Schema",
    description:
      "Checks that a data schema descriptor is well-formed and conforms to the Fairspec specification. Ensures that all field definitions use valid types and that constraints are properly structured.",
  },
  {
    icon: Search,
    title: "Infer Dataset",
    description:
      "Scans a directory of files and produces a dataset descriptor capturing the structure, resources, and metadata. Automatically discovers tabular files, infers their schemas and dialects.",
  },
  {
    icon: CheckCircle2,
    title: "Validate Dataset",
    description:
      "Checks that a dataset descriptor is well-formed and that all referenced resources are consistent. Verifies that resource paths resolve correctly and that schemas match the actual data.",
  },
  {
    icon: ListOrdered,
    title: "Query Dataset",
    description:
      "Runs a SQL query across a dataset, with each table resource available under its name, and returns the matching rows. Joins related resources in one query and loads only the resources the query names.",
  },
  {
    icon: Eye,
    title: "Preview File",
    description:
      "Returns the raw source of a local or remote file, exactly as stored and before any parsing, so a CSV's delimiter, header and quoting are visible. Reads only the first lines and caps the size, so a large file cannot flood the conversation.",
  },
  {
    icon: Search,
    title: "Infer File Dialect",
    description:
      "Detects the dialect of a tabular file, including delimiter, quote character, and header row presence. Samples the file content to reliably determine formatting details such as line terminators and encoding.",
  },
  {
    icon: ListOrdered,
    title: "Query Table",
    description:
      "Runs a SQL query against a tabular data file and returns the matching rows. Supports standard SQL syntax for filtering, sorting, aggregating, and joining data.",
  },
  {
    icon: CheckCircle2,
    title: "Validate Table",
    description:
      "Checks that a table descriptor is well-formed and that the referenced data matches the declared schema. Reports mismatches between expected and observed field types, missing values, and constraint violations.",
  },
  {
    icon: Search,
    title: "Infer Table Schema",
    description:
      "Reads a tabular data file and infers a table schema descriptor with field names, types, and constraints. Analyzes column values to determine appropriate data types and identifies common patterns.",
  },
  {
    icon: CheckCircle2,
    title: "Validate Table Schema",
    description:
      "Checks that a table schema descriptor is well-formed and conforms to the Fairspec specification. Ensures that all field definitions, types, and constraints are valid and properly structured.",
  },
]

function Tools() {
  return (
    <LandingSection
      tinted
      title="Tools"
      lede="Twelve tools organized around core Fairspec concepts: data, data schemas, datasets, files, file dialects, tables, and table schemas."
    >
      <FeatureGrid features={tools} />
    </LandingSection>
  )
}

/* ─────────────────────────── Samples ─────────────────────────── */

function TableSchemaSample() {
  return (
    <code className={tk.body}>
      <span className={tk.punct}>{"{"}</span>
      {"\n  "}
      <span className={tk.key}>"id"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{ "}</span>
      <span className={tk.key}>"type"</span>
      <span className={tk.punct}>:</span> <span className={tk.str}>"integer"</span>
      <span className={tk.punct}>{" }"}</span>
      <span className={tk.punct}>,</span>
      {"\n  "}
      <span className={tk.key}>"name"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{ "}</span>
      <span className={tk.key}>"type"</span>
      <span className={tk.punct}>:</span> <span className={tk.str}>"string"</span>
      <span className={tk.punct}>{" }"}</span>
      <span className={tk.punct}>,</span>
      {"\n  "}
      <span className={tk.key}>"joined"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{ "}</span>
      <span className={tk.key}>"type"</span>
      <span className={tk.punct}>:</span> <span className={tk.str}>"string"</span>
      <span className={tk.punct}>,</span> <span className={tk.key}>"format"</span>
      <span className={tk.punct}>:</span> <span className={tk.str}>"date"</span>
      <span className={tk.punct}>{" }"}</span>
      <span className={tk.punct}>,</span>
      {"\n  "}
      <span className={tk.key}>"score"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{ "}</span>
      <span className={tk.key}>"type"</span>
      <span className={tk.punct}>:</span> <span className={tk.str}>"number"</span>
      <span className={tk.punct}>{" }"}</span>
      {"\n"}
      <span className={tk.punct}>{"}"}</span>
    </code>
  )
}
