import { Link, createFileRoute } from "@tanstack/react-router"
import {
  BookOpen,
  Bot,
  Code,
  CodeXml,
  FileText,
  Github,
  Globe,
  Laptop,
  Puzzle,
  Rocket,
  ScrollText,
  Sparkles,
  SquareTerminal,
} from "lucide-react"
import type { ReactNode } from "react"
import { BackToTop } from "livemark/components/BackToTop"
import { Banner } from "livemark/components/Banner"
import { Header } from "livemark/components/Header"
import {
  Datist,
  FeatureGrid,
  FinalCta,
  Hero,
  Highlight,
  type InstallCommand,
  LandingSection,
  type Point,
  ShowcaseSection,
  type Slide,
  SplitSection,
  WindowCard,
  actionClass,
  type Benchmark,
  BenchmarkCard,
  textLinkClass,
  tk,
  ShotCard,
} from "../components/Landing.tsx"
import { InferredDataset, MembersCsv, shots, WindowCode } from "../components/Panes.tsx"

// @ts-ignore
export const Route = createFileRoute("/")({
  component: Landing,
})

function Landing() {
  return (
    <>
      <Banner />
      <Header />
      <main className="flex flex-col">
        <Hero
          eyebrow="Open source · MIT License"
          title={
            <>
              Standard and software to make <Highlight>data FAIR</Highlight>
            </>
          }
          lede={
            <>
              Data exchange format compatible with{" "}
              <code className="font-mono text-foreground">DataCite</code> for metadata and{" "}
              <code className="font-mono text-foreground">JSON Schema</code> for
              structured data, shipped with Python, TypeScript, Rust, an MCP server, agent
              skills and a desktop app.
            </>
          }
          actions={
            <>
              <Link
                to="/$"
                params={{ _splat: "overview" }}
                className={actionClass("default")}
              >
                <Rocket className="size-4" />
                Get started
              </Link>
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
          install={install}
          slides={slides}
          aside={<BenchmarkCard benchmark={benchmark} />}
        />
        <SplitSection
          title="A simple format for FAIR data"
          lede="Built on widely-adopted open standards. Easy to adopt, easy to extend."
          points={standardPoints}
          visual={
            <WindowCard label="dataset.json">
              <DatasetSample />
            </WindowCard>
          }
          links={
            <Link to="/$" params={{ _splat: "specs" }} className={textLinkClass}>
              Read the specifications →
            </Link>
          }
        />
        <LandingSection
          title="Tools for every workflow"
          lede="Adopt the standard from your language of choice, your terminal, your AI assistant, or a desktop interface."
        >
          <FeatureGrid features={tools} />
        </LandingSection>
        <ShowcaseSection
          title="One command. A FAIR dataset."
          lede="Point Fairspec at your files and it describes them: dialect, schema and integrity, ready to cite and validate."
          left={{
            label: "members.csv",
            content: (
              <WindowCode>
                <MembersCsv />
              </WindowCode>
            ),
          }}
          right={{
            label: "fairspec dataset infer members.csv",
            content: (
              <WindowCode>
                <InferredDataset />
              </WindowCode>
            ),
          }}
        />
        <SplitSection
          reverse
          title="Ready for AI agents"
          lede="Hand the busywork of describing and checking data to your coding agent or AI assistant."
          points={agentPoints}
          visual={<ShotCard shot={shots.skills} />}
          links={
            <>
              <Link
                to="/$"
                params={{ _splat: "python/agent-skills" }}
                className={textLinkClass}
              >
                Install the agent skills →
              </Link>
              <Link to="/mcp-server/" className={textLinkClass}>
                Connect the MCP server →
              </Link>
            </>
          }
        />
        <FinalCta
          title={
            <>
              Describe your data. <span className="text-primary">Make it FAIR.</span>
            </>
          }
          lede="Read the specifications, browse the examples, and start describing your datasets in minutes."
          actions={
            <>
              <Link
                to="/$"
                params={{ _splat: "overview" }}
                className={actionClass("default")}
              >
                <BookOpen className="size-4" />
                Read the specs
              </Link>
              <Link to="/cli-tool/" className={actionClass("ghost")}>
                <SquareTerminal className="size-4" />
                Use the CLI tool
              </Link>
              <Link
                to="/$"
                params={{ _splat: "python/agent-skills" }}
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

const install: InstallCommand[] = [
  { command: "curl -fsSL", target: "https://fairspec.org/install.sh | sh" },
  { command: "pip install", target: "fairspec" },
  { command: "npm install", target: "fairspec" },
  { command: "cargo install", target: "fairspec" },
]

const slides: Slide[] = [
  { id: "standard", title: "Standard", content: <SpecPreview /> },
  { id: "cli", title: "CLI", shot: shots.validate },
  { id: "mcp", title: "MCP", shot: shots.mcpValidate },
  { id: "skills", title: "Skills", shot: shots.skills },
  { id: "desktop", title: "Desktop", shot: shots.desktop },
]

// nyc-311 10GB, full validators only; Fairspec is its best run (Rust, 37def29).
const benchmark: Benchmark = {
  title: "Validating a 10 GB CSV",
  caption: "19.4M rows · 41 columns",
  rows: [
    { tool: "Fairspec", seconds: 11.5, ours: true },
    { tool: "pandera (DuckDB)", seconds: 64.9 },
    { tool: "qsv", seconds: 71.4 },
    { tool: "CSV Validator", seconds: 530.3 },
    { tool: "frictionless-py", seconds: 787.7 },
  ],
  footnote: "Four more validators ran out of memory under a 10 GB cap.",
}

const agentPoints: Point[] = [
  {
    icon: Bot,
    title: "Agent skills",
    description: (
      <>
        <code className="font-mono text-foreground">
          npx skills add https://fairspec.org
        </code>{" "}
        teaches Claude Code, Cursor or Codex to explore, infer and validate data with
        Fairspec.
      </>
    ),
  },
  {
    icon: Sparkles,
    title: "MCP server",
    description:
      "Gives any MCP-capable assistant tools to validate data, infer schemas and query tables, running on your machine.",
  },
  {
    icon: SquareTerminal,
    title: "A CLI built for scripts",
    description: (
      <>
        Commands take <code className="font-mono text-foreground">--json</code>, so an
        agent with a shell reads results as data, not prose.
      </>
    ),
  },
  {
    icon: ScrollText,
    title: "Docs agents can read",
    description: (
      <>
        Every page has a Markdown twin, indexed in{" "}
        <code className="font-mono text-foreground">/llms.txt</code>.
      </>
    ),
  },
]

/* ─────────────────────────── Standard ─────────────────────────── */

const standardPoints: Point[] = [
  {
    icon: BookOpen,
    title: "Standards-based",
    description:
      "Compatible with DataCite Metadata Schema 4.6 for citation and JSON Schema Draft 2020-12 for structural validation.",
  },
  {
    icon: FileText,
    title: "Format-agnostic",
    description:
      "Describes any kind of data — CSV, TSV, JSON, JSONL, Parquet, Arrow, XLSX, ODS, SQLite — through a unified file dialect layer.",
  },
  {
    icon: Globe,
    title: "FAIR by design",
    description:
      "Findable, Accessible, Interoperable, and Reusable. Datasets carry the metadata needed to be properly cited and discovered.",
  },
  {
    icon: Puzzle,
    title: "Extensible",
    description:
      "Domain-specific profiles add custom properties and validation rules while staying compatible with the base specification.",
  },
]

/* ─────────────────────────── Tools ─────────────────────────── */

const tools: Point[] = [
  {
    icon: Code,
    title: "Python",
    description:
      "Validate, infer, and query Fairspec descriptors from Python, with first-class support for pandas and Polars DataFrames and custom checks in SQL or Polars.",
    link: { to: "/$", splat: "python/getting-started" },
  },
  {
    icon: CodeXml,
    title: "TypeScript",
    description:
      "The same API surface as the Python package, built on Polars, and shipping a terminal CLI alongside the library.",
    link: { to: "/$", splat: "typescript/getting-started" },
  },
  {
    icon: SquareTerminal,
    title: "CLI Tool",
    description:
      "Validate datasets, infer schemas, and query tables from scripts, CI pipelines, or an interactive terminal.",
    link: { to: "/cli-tool/" },
  },
  {
    icon: Sparkles,
    title: "MCP Server",
    description:
      "Connects AI assistants to the framework over the Model Context Protocol, so you can validate and infer in plain language.",
    link: { to: "/mcp-server/" },
  },
  {
    icon: Bot,
    title: "Agent Skills",
    description:
      "Teach coding agents when and how to explore, infer and validate data with the CLI, MCP server or libraries.",
    link: { to: "/$", splat: "python/agent-skills" },
  },
  {
    icon: Laptop,
    title: "Desktop",
    description:
      "Validate and infer descriptors through a visual interface, with your files never leaving your machine.",
    link: { to: "/desktop/" },
  },
]

/* ─────────────────────────── Samples ─────────────────────────── */

function SpecPreview() {
  return (
    <div className="space-y-3 px-3 py-4 font-mono text-[10.5px] leading-relaxed sm:p-5 sm:text-sm xl:p-8 xl:text-base">
      <SpecLabel>DataCite compatible metadata</SpecLabel>
      <pre className="overflow-x-auto">
        <DataciteLines />
      </pre>
      <SpecLabel>JSONSchema compatible for the data</SpecLabel>
      <pre className="overflow-x-auto">
        <TableSchemaLines />
      </pre>
    </div>
  )
}

function SpecLabel(props: { children: ReactNode }) {
  return (
    <div className="inline-flex items-center rounded-md border border-border bg-muted/60 px-2 py-0.5 text-xs text-muted-foreground">
      {props.children}
    </div>
  )
}

function DataciteLines() {
  return (
    <code className={tk.body}>
      <span className={tk.key}>"doi"</span>
      <span className={tk.punct}>:</span>{" "}
      <span className={tk.str}>"10.5281/zenodo.1234567"</span>
      <span className={tk.punct}>,</span>
      {"\n"}
      <span className={tk.key}>"titles"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>[{"{ "}</span>
      <span className={tk.key}>"title"</span>
      <span className={tk.punct}>:</span>{" "}
      <span className={tk.str}>"Climate Survey 2025"</span>
      <span className={tk.punct}>{" }"}],</span>
      {"\n"}
      <span className={tk.key}>"creators"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>[{"{ "}</span>
      <span className={tk.key}>"name"</span>
      <span className={tk.punct}>:</span> <span className={tk.str}>"Ada Lovelace"</span>
      <span className={tk.punct}>{" }"}]</span>
    </code>
  )
}

function TableSchemaLines() {
  return (
    <code className={tk.body}>
      <span className={tk.key}>"tableSchema"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{"}</span>
      {"\n  "}
      <span className={tk.key}>"properties"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{"}</span>
      {"\n    "}
      <span className={tk.key}>"id"</span>
      <span className={tk.punct}>:</span> <span className={tk.punct}>{"{ "}</span>
      <span className={tk.key}>"type"</span>
      <span className={tk.punct}>:</span> <span className={tk.str}>"integer"</span>
      <span className={tk.punct}>{" }"},</span>
      {"\n    "}
      <span className={tk.key}>"temperature"</span>
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

function DatasetSample() {
  return (
    <pre className="p-5 text-sm leading-relaxed font-mono overflow-x-auto">
      <code className={tk.body}>
        <span className={tk.punct}>{"{"}</span>
        {"\n  "}
        <span className={tk.key}>"$schema"</span>
        <span className={tk.punct}>:</span>{" "}
        <span className={tk.url}>
          "https://fairspec.org/profiles/latest/dataset.json"
        </span>
        <span className={tk.punct}>,</span>
        {"\n  "}
        <span className={tk.key}>"title"</span>
        <span className={tk.punct}>:</span>{" "}
        <span className={tk.str}>"Climate Survey 2025"</span>
        <span className={tk.punct}>,</span>
        {"\n  "}
        <span className={tk.key}>"creators"</span>
        <span className={tk.punct}>:</span> <span className={tk.punct}>[</span>
        {"\n    "}
        <span className={tk.punct}>{"{"}</span> <span className={tk.key}>"name"</span>
        <span className={tk.punct}>:</span> <span className={tk.str}>"Ada Lovelace"</span>{" "}
        <span className={tk.punct}>{"}"}</span>
        {"\n  "}
        <span className={tk.punct}>],</span>
        {"\n  "}
        <span className={tk.key}>"resources"</span>
        <span className={tk.punct}>:</span> <span className={tk.punct}>[</span>
        {"\n    "}
        <span className={tk.punct}>{"{"}</span>
        {"\n      "}
        <span className={tk.key}>"data"</span>
        <span className={tk.punct}>:</span>{" "}
        <span className={tk.str}>"measurements.csv"</span>
        <span className={tk.punct}>,</span>
        {"\n      "}
        <span className={tk.key}>"fileDialect"</span>
        <span className={tk.punct}>:</span> <span className={tk.punct}>{"{"}</span>{" "}
        <span className={tk.key}>"format"</span>
        <span className={tk.punct}>:</span> <span className={tk.str}>"csv"</span>{" "}
        <span className={tk.punct}>{"}"},</span>
        {"\n      "}
        <span className={tk.key}>"tableSchema"</span>
        <span className={tk.punct}>:</span> <span className={tk.punct}>{"{"}</span>
        {"\n        "}
        <span className={tk.key}>"primaryKey"</span>
        <span className={tk.punct}>:</span> <span className={tk.punct}>[</span>
        <span className={tk.str}>"id"</span>
        <span className={tk.punct}>],</span>
        {"\n        "}
        <span className={tk.key}>"properties"</span>
        <span className={tk.punct}>:</span> <span className={tk.punct}>{"{"}</span>
        {"\n          "}
        <span className={tk.key}>"id"</span>
        <span className={tk.punct}>:</span> <span className={tk.punct}>{"{"}</span>{" "}
        <span className={tk.key}>"type"</span>
        <span className={tk.punct}>:</span> <span className={tk.str}>"integer"</span>{" "}
        <span className={tk.punct}>{"}"},</span>
        {"\n          "}
        <span className={tk.key}>"temperature"</span>
        <span className={tk.punct}>:</span> <span className={tk.punct}>{"{"}</span>{" "}
        <span className={tk.key}>"type"</span>
        <span className={tk.punct}>:</span> <span className={tk.str}>"number"</span>{" "}
        <span className={tk.punct}>{"}"}</span>
        {"\n        "}
        <span className={tk.punct}>{"}"}</span>
        {"\n      "}
        <span className={tk.punct}>{"}"}</span>
        {"\n    "}
        <span className={tk.punct}>{"}"}</span>
        {"\n  "}
        <span className={tk.punct}>]</span>
        {"\n"}
        <span className={tk.punct}>{"}"}</span>
      </code>
    </pre>
  )
}
