import { Link, createFileRoute } from "@tanstack/react-router"
import {
  ArrowRight,
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
import type { ComponentType, SVGProps } from "react"
import { BackToTop } from "livemark/components/BackToTop"
import { Banner } from "livemark/components/Banner"
import { Header } from "livemark/components/Header"
import {
  Datist,
  FeatureGrid,
  FinalCta,
  Hero,
  Highlight,
  LandingSection,
  type Point,
  PointList,
  Reveal,
  type Slide,
  WindowCard,
  actionClass,
  tk,
} from "../components/Landing.tsx"
import {
  BrokenCsv,
  ChatPane,
  DesktopPane,
  Pane,
  PythonPane,
  SkillsPane,
  TypeScriptPane,
  ValidatePane,
  ValidationReport,
} from "../components/Panes.tsx"

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
          eyebrow="Technical preview"
          title={
            <>
              Make your data <Highlight>FAIR</Highlight>
            </>
          }
          lede={
            <>
              Data exchange format compatible with{" "}
              <code className="font-mono text-foreground">DataCite</code> for metadata and{" "}
              <code className="font-mono text-foreground">JSON Schema</code> for
              structured data, shipped with Python, TypeScript, an MCP server, agent
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
          slides={slides}
        />
        <Standard />
        <Showcase />
        <Tools />
        <Agents />
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

const brokenCsv = { eyebrow: "The same input", file: "broken.csv", code: <BrokenCsv /> }

const slides: Slide[] = [
  {
    id: "standard",
    title: "Standard",
    pane: (
      <Pane label="dataset.json">
        <p className="mb-2 text-xs text-muted-foreground">DataCite compatible metadata</p>
        <pre>
          <DataciteLines />
        </pre>
        <p className="mt-4 mb-2 text-xs text-muted-foreground">
          JSON Schema compatible data
        </p>
        <pre>
          <TableSchemaLines />
        </pre>
      </Pane>
    ),
    source: {
      eyebrow: "Checked by a profile",
      file: "$schema",
      code: <span className={tk.url}>fairspec.org/profiles/latest/dataset.json</span>,
    },
  },
  { id: "python", title: "Python", pane: <PythonPane />, source: brokenCsv },
  { id: "typescript", title: "TypeScript", pane: <TypeScriptPane />, source: brokenCsv },
  { id: "cli", title: "CLI", pane: <ValidatePane />, source: brokenCsv },
  {
    id: "mcp",
    title: "MCP",
    pane: (
      <ChatPane prompt="Check broken.csv against schema.json" tool="validateTableTool">
        <ValidationReport />
      </ChatPane>
    ),
    source: brokenCsv,
  },
  {
    id: "skills",
    title: "Skills",
    pane: <SkillsPane />,
    source: {
      eyebrow: "Three agent skills",
      file: "fairspec.org/.well-known/agent-skills/",
      code: (
        <>
          fairspec-exploration{"\n"}fairspec-inference{"\n"}fairspec-validation
        </>
      ),
    },
  },
  {
    id: "desktop",
    title: "Desktop",
    pane: <DesktopPane />,
    source: {
      eyebrow: "No code required",
      file: "macOS · Windows · Linux",
      code: <>Your files never leave your machine.</>,
    },
  },
]

/* ─────────────────────────── Features ─────────────────────────── */

const features: Point[] = [
  {
    icon: Rocket,
    title: "Lightweight",
    description:
      "A few simple JSON-based metadata formats for describing catalogs, datasets, tables, and files. No bespoke vocabularies to learn.",
  },
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
  {
    icon: Laptop,
    title: "Software-first",
    description:
      "Python and TypeScript implementations out-of-the-box, plus an MCP server and agent skills so AI assistants can work with Fairspec data.",
  },
]

const specs = [
  { title: "Catalog", splat: "specs/catalog" },
  { title: "Dataset", splat: "specs/dataset" },
  { title: "Table Schema", splat: "specs/table-schema" },
  { title: "Data Schema", splat: "specs/data-schema" },
  { title: "File Dialect", splat: "specs/file-dialect" },
  { title: "Extensions", splat: "extensions" },
]

function Standard() {
  return (
    <LandingSection
      title="A simple format for FAIR data"
      lede="Built on widely-adopted open standards. Easy to adopt, easy to extend."
    >
      <FeatureGrid features={features} />
      <Reveal>
        <div className="mt-10 flex flex-wrap items-center justify-center gap-2 md:mt-12">
          <span className="mr-2 text-sm text-muted-foreground">Read the specs:</span>
          {specs.map(spec => (
            <Link
              key={spec.splat}
              to="/$"
              params={{ _splat: spec.splat }}
              className="rounded-full border border-border bg-card px-3 py-1 text-sm font-medium text-foreground no-underline transition-colors hover:border-primary/40 hover:text-primary"
            >
              {spec.title}
            </Link>
          ))}
        </div>
      </Reveal>
    </LandingSection>
  )
}

/* ─────────────────────────── Showcase ─────────────────────────── */

function Showcase() {
  return (
    <LandingSection
      tinted
      title="One JSON file. A FAIR dataset."
      lede="A Fairspec descriptor carries everything a consumer needs: citation metadata, file dialects, and validated schemas."
    >
      <Reveal>
        <WindowCard label="dataset.json" className="max-w-4xl mx-auto">
          <DatasetSample />
        </WindowCard>
      </Reveal>
    </LandingSection>
  )
}

/* ─────────────────────────── Agents ─────────────────────────── */

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

function Agents() {
  return (
    <LandingSection
      title="Ready for AI agents"
      lede="Hand the busywork of describing and checking data to your coding agent or AI assistant."
    >
      <div className="grid grid-cols-1 md:grid-cols-2 gap-10 md:gap-12 items-center">
        <Reveal>
          <div className="rounded-card border border-primary/20 bg-card overflow-hidden">
            <SkillsPane />
          </div>
        </Reveal>
        <PointList points={agentPoints} />
      </div>
      <Reveal>
        <div className="mt-10 flex flex-wrap justify-center gap-x-8 gap-y-3 md:mt-12">
          <Link
            to="/$"
            params={{ _splat: "python/agent-skills" }}
            className="text-sm font-medium text-primary underline-offset-4 hover:underline"
          >
            Install the agent skills →
          </Link>
          <Link
            to="/mcp-server/"
            className="text-sm font-medium text-primary underline-offset-4 hover:underline"
          >
            Connect the MCP server →
          </Link>
        </div>
      </Reveal>
    </LandingSection>
  )
}

/* ─────────────────────────── Tools ─────────────────────────── */

interface Implementation {
  icon: ComponentType<SVGProps<SVGSVGElement>>
  title: string
  description: string
  command: string
  link: { to: "/$"; splat: string } | { to: string }
}

const implementations: Implementation[] = [
  {
    icon: Code,
    title: "Python",
    description:
      "Validate, infer, and query Fairspec descriptors from Python, with first-class support for pandas and Polars DataFrames.",
    command: "pip install fairspec",
    link: { to: "/$", splat: "python/getting-started" },
  },
  {
    icon: CodeXml,
    title: "TypeScript",
    description:
      "The same API surface as the Python package, built on Polars, and shipping a terminal CLI alongside the library.",
    command: "npm install fairspec",
    link: { to: "/$", splat: "typescript/getting-started" },
  },
  {
    icon: SquareTerminal,
    title: "CLI Tool",
    description:
      "Validate datasets, infer schemas, and query tables from scripts, CI pipelines, or an interactive terminal.",
    command: "uv tool install fairspec",
    link: { to: "/cli-tool/" },
  },
  {
    icon: Sparkles,
    title: "MCP Server",
    description:
      "Connects AI assistants to the framework over the Model Context Protocol, so you can validate and infer in plain language.",
    command: "npx fairspec@latest mcp",
    link: { to: "/mcp-server/" },
  },
  {
    icon: Bot,
    title: "Agent Skills",
    description:
      "Teach coding agents when and how to explore, infer and validate data with the CLI, MCP server or libraries.",
    command: "npx skills add https://fairspec.org",
    link: { to: "/$", splat: "python/agent-skills" },
  },
  {
    icon: Laptop,
    title: "Desktop",
    description:
      "Validate and infer descriptors through a visual interface, with your files never leaving your machine.",
    command: "Download for macOS, Windows, Linux",
    link: { to: "/desktop/" },
  },
]

function Tools() {
  return (
    <LandingSection
      tinted
      title="Tools for every workflow"
      lede="Adopt the standard from your language of choice, your terminal, your AI assistant, or a desktop interface."
    >
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {implementations.map((impl, i) => (
          <Reveal key={impl.title} delayMs={i * 60}>
            <ImplementationCard {...impl} />
          </Reveal>
        ))}
      </div>
    </LandingSection>
  )
}

function ImplementationCard(props: Implementation) {
  const { icon: Icon, title, description, command, link } = props
  const className =
    "group flex flex-col h-full rounded-card border border-border bg-card p-6 no-underline transition-all hover:border-primary/40 hover:-translate-y-0.5 hover:shadow-lg motion-reduce:transition-none motion-reduce:hover:translate-y-0"
  const body = (
    <>
      <div className="flex items-center gap-3 mb-3">
        <div className="inline-flex items-center justify-center size-10 rounded-lg bg-primary/10 text-primary">
          <Icon className="size-5" />
        </div>
        <h3 className="text-lg font-semibold text-foreground">{title}</h3>
        <ArrowRight className="size-4 ml-auto text-muted-foreground transition-transform group-hover:translate-x-0.5" />
      </div>
      <p className="text-sm text-muted-foreground leading-relaxed">{description}</p>
      <div className="mt-auto pt-4">
        <div className="truncate rounded-md border border-border bg-background px-3 py-2 font-mono text-xs text-muted-foreground">
          {command}
        </div>
      </div>
    </>
  )
  if ("splat" in link) {
    return (
      <Link to="/$" params={{ _splat: link.splat }} className={className}>
        {body}
      </Link>
    )
  }
  return (
    <Link to={link.to} className={className}>
      {body}
    </Link>
  )
}

/* ─────────────────────────── Samples ─────────────────────────── */

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
