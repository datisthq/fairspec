import { Link, createFileRoute } from "@tanstack/react-router"
import {
  Bot,
  CheckCircle2,
  Cpu,
  Eye,
  Github,
  Laptop,
  ListOrdered,
  MessageSquare,
  Rocket,
  ScrollText,
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
  type Language,
  LandingSection,
  LanguageTabs,
  type Point,
  ShowcaseSection,
  type Slide,
  SplitSection,
  WindowCard,
  actionClass,
  textLinkClass,
  tk,
} from "../components/Landing.tsx"
import {
  BrokenCsv,
  InferredDataset,
  MembersCsv,
  shots,
  WindowCode,
} from "../components/Panes.tsx"

// @ts-ignore
export const Route = createFileRoute("/mcp-server/")({
  component: McpServerPage,
  head: () => ({
    meta: [
      { title: "Fairspec MCP" },
      {
        name: "description",
        content: "AI assistant integration for the Fairspec data management framework.",
      },
    ],
  }),
})

function McpServerPage() {
  const [language, setLanguage] = useState<Language>("python")
  return (
    <>
      <Banner />
      <Header />
      <main className="flex flex-col">
        <Hero
          eyebrow="For Python, TypeScript and Rust"
          title="Fairspec MCP"
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
          slides={slides}
        />
        <SplitSection
          id="installation"
          title="Connect in one step"
          lede="Wire up the Fairspec MCP server in your favorite AI assistant. It ships with the Python, TypeScript and Rust packages — pick the one you have."
          switcher={<LanguageTabs language={language} onChange={setLanguage} />}
          points={connectPoints}
          visual={
            <WindowCard label="Install">
              <Installation language={language} />
            </WindowCard>
          }
        />
        <LandingSection
          title="Tools"
          lede="Twelve tools organized around core Fairspec concepts: data, data schemas, datasets, files, file dialects, tables, and table schemas."
        >
          <FeatureGrid features={tools} />
        </LandingSection>
        <ShowcaseSection
          title="Files in, a FAIR dataset out"
          lede="Ask for a dataset and the server describes your files: dialect, schema and integrity, ready to cite and validate."
          left={{
            label: "members.csv",
            content: (
              <WindowCode>
                <MembersCsv />
              </WindowCode>
            ),
          }}
          right={{
            label: "inferDatasetTool",
            content: (
              <WindowCode>
                <InferredDataset />
              </WindowCode>
            ),
          }}
        />
        <SplitSection
          reverse
          title="Ask in plain language"
          lede="Just ask in natural language. Try one of these prompts."
          points={askPoints}
          visual={
            <WindowCard label="prompts">
              <Prompts />
            </WindowCard>
          }
          links={
            <>
              <Link
                to="/$"
                params={{ _splat: "python/agent-skills" }}
                className={textLinkClass}
              >
                Install the agent skills →
              </Link>
              <Link to="/cli-tool/" className={textLinkClass}>
                Use the CLI tool →
              </Link>
            </>
          }
        />
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
              <Link
                to="/$"
                params={{ _splat: "python/agent-skills" }}
                className={actionClass("default")}
              >
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

const membersCsv = { eyebrow: "Input", file: "members.csv", code: <MembersCsv /> }

const slides: Slide[] = [
  {
    id: "validate",
    title: "Validate",
    shot: shots.mcpValidate,
    source: { eyebrow: "Input", file: "broken.csv", code: <BrokenCsv /> },
  },
  { id: "infer", title: "Infer", shot: shots.mcpInfer, source: membersCsv },
  { id: "query", title: "Query", shot: shots.mcpQuery, source: membersCsv },
]

/* ─────────────────────────── Installation ─────────────────────────── */

const launchers: Record<Language, { command: string; args: string[] }> = {
  python: { command: "uvx", args: ["fairspec@latest", "mcp"] },
  typescript: { command: "npx", args: ["fairspec@latest", "mcp"] },
  rust: { command: "fairspec", args: ["mcp"] },
}

function BashAgentAdd({
  agent,
  language,
}: {
  agent: "claude" | "codex"
  language: Language
}) {
  const launcher = launchers[language]
  return (
    <code className={tk.body}>
      <span className={tk.cmd}>{agent}</span> mcp add{" "}
      <span className={tk.str}>fairspec</span> <span className={tk.punct}>--</span>{" "}
      {launcher.command}
      {launcher.args.map(arg => (
        <span key={arg}>
          {" "}
          {arg.includes("@") ? <span className={tk.str}>{arg}</span> : arg}
        </span>
      ))}
    </code>
  )
}

function McpServersJson({
  rootKey,
  language,
}: {
  rootKey: "mcpServers" | "servers"
  language: Language
}) {
  const launcher = launchers[language]
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
      <span className={tk.str}>"{launcher.command}"</span>
      <span className={tk.punct}>,</span>
      {"\n      "}
      <span className={tk.key}>"args"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>[</span>
      {launcher.args.map((arg, i) => (
        <span key={arg}>
          {i > 0 ? <span className={tk.punct}>, </span> : null}
          <span className={tk.str}>"{arg}"</span>
        </span>
      ))}
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
  snippet: (language: Language) => ReactNode
}

const assistants: Assistant[] = [
  {
    name: "Claude Desktop",
    description: (
      <>
        Add to your <code>claude_desktop_config.json</code>:
      </>
    ),
    snippet: language => <McpServersJson rootKey="mcpServers" language={language} />,
  },
  {
    name: "Claude Code",
    description: <>Add the server with one command in any project directory.</>,
    snippet: language => <BashAgentAdd agent="claude" language={language} />,
  },
  {
    name: "Codex",
    description: (
      <>
        One command configures the Codex CLI, the ChatGPT desktop app, and the IDE
        extension — they share the same MCP configuration.
      </>
    ),
    snippet: language => <BashAgentAdd agent="codex" language={language} />,
  },
  {
    name: "Gemini CLI",
    description: (
      <>
        Add to <code>~/.gemini/settings.json</code>:
      </>
    ),
    snippet: language => <McpServersJson rootKey="mcpServers" language={language} />,
  },
]

const connectPoints: Point[] = [
  {
    icon: Cpu,
    title: "Python, TypeScript or Rust",
    description: (
      <>
        Run it with <code className="font-mono text-foreground">uvx</code> from the Python
        package, <code className="font-mono text-foreground">npx</code> from the
        TypeScript one, or the <code className="font-mono text-foreground">fairspec</code>{" "}
        binary from the GitHub releases. Same tools, same names.
      </>
    ),
  },
  {
    icon: MessageSquare,
    title: "Your assistant",
    description:
      "Claude Desktop, Claude Code, Codex and Gemini CLI each take one command or one config entry.",
  },
  {
    icon: Laptop,
    title: "Runs on your machine",
    description: (
      <>
        The server is a local process your assistant starts. It reads the files you point
        it at and fetches only public URLs; set{" "}
        <code className="font-mono text-foreground">FAIRSPEC_HOST_POLICY=any</code> to
        reach localhost or intranet hosts.
      </>
    ),
  },
  {
    icon: Wrench,
    title: "Twelve tools",
    description:
      "Validate, infer, preview and query data, schemas, dialects, tables and whole datasets.",
  },
]

function Installation(props: { language: Language }) {
  const [active, setActive] = useState(0)
  const assistant = assistants[active] ?? assistants[0]
  if (!assistant) return null
  return (
    <>
      <div role="tablist" className="flex flex-wrap border-b border-border">
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
      <div className="min-w-0 p-5">
        <p className="text-sm text-muted-foreground mb-4">{assistant.description}</p>
        <pre className="rounded-md border border-border bg-background p-4 text-sm leading-relaxed font-mono overflow-x-auto">
          {assistant.snippet(props.language)}
        </pre>
        {props.language === "rust" ? (
          <p className="mt-4 text-sm text-muted-foreground">
            Install the <code>fairspec</code> binary first:{" "}
            <code>curl -fsSL https://fairspec.org/install.sh | sh -s -- terminal</code>
          </p>
        ) : null}
      </div>
    </>
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

const askPoints: Point[] = [
  {
    icon: Bot,
    title: "Skills pick the tool",
    description:
      "The Fairspec agent skills tell your coding agent which tool fits a task and how to read what it returns.",
  },
  {
    icon: SquareTerminal,
    title: "The same operations in the CLI",
    description:
      "Every tool has a fairspec command behind it, so a script or CI job can repeat what the assistant did.",
  },
  {
    icon: ScrollText,
    title: "Docs agents can read",
    description: (
      <>
        Every docs page has a Markdown twin, indexed in{" "}
        <code className="font-mono text-foreground">/llms.txt</code>.
      </>
    ),
  },
]

function Prompts() {
  return (
    <ul className="divide-y divide-border">
      {prompts.map(p => (
        <li key={p} className="flex items-start gap-3 px-5 py-3">
          <Sparkles className="size-4 mt-0.5 text-primary shrink-0" />
          <span className="min-w-0 text-sm text-foreground leading-relaxed">{p}</span>
        </li>
      ))}
    </ul>
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
      "Checks that a table descriptor is well-formed and that the referenced data matches the declared schema. Reports mismatches between expected and observed field types, missing values, and constraint violations, plus any named SQL checks you add.",
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
