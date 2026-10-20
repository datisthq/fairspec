import { Link, createFileRoute } from "@tanstack/react-router"
import {
  BookOpen,
  Bot,
  Code,
  Download,
  Github,
  Globe,
  ListChecks,
  Search,
  ShieldCheck,
  Sparkles,
  SquareTerminal,
  Wand2,
} from "lucide-react"
import type { ComponentType, ReactNode, SVGProps } from "react"
import { BackToTop } from "livemark/components/BackToTop"
import { Banner } from "livemark/components/Banner"
import { Header } from "livemark/components/Header"
import {
  Datist,
  FinalCta,
  Hero,
  Highlight,
  LandingSection,
  type Point,
  PointList,
  Reveal,
  WindowCard,
  actionClass,
  tk,
} from "../components/Landing.tsx"

// @ts-ignore
export const Route = createFileRoute("/agent-skills/")({
  component: AgentSkillsPage,
  head: () => ({
    meta: [
      { title: "Fairspec Agent Skills" },
      {
        name: "description",
        content:
          "Agent skills that teach coding agents to explore, infer and validate data with Fairspec.",
      },
    ],
  }),
})

function AgentSkillsPage() {
  return (
    <>
      <Banner />
      <Header />
      <main className="flex flex-col">
        <Hero
          eyebrow="Agent Skills"
          title={
            <>
              Teach your agent <Highlight>Fairspec</Highlight>
            </>
          }
          lede="Skills that tell Claude Code, Cursor, Codex and other coding agents how to explore, infer and validate data with Fairspec, through the CLI, the MCP server or the Python and TypeScript libraries."
          actions={
            <>
              <a href="#install" className={actionClass("default")}>
                <Download className="size-4" />
                Install
              </a>
              <a
                href="https://github.com/datisthq/fairspec/tree/main/.skills"
                target="_blank"
                rel="noopener noreferrer"
                className={actionClass("outline")}
              >
                <Github className="size-4" />
                View source
              </a>
            </>
          }
          visual={
            <WindowCard label="terminal">
              <HeroTerminal />
            </WindowCard>
          }
        />
        <Skills />
        <Install />
        <HowItWorks />
        <FinalCta
          title={
            <>
              Ask in plain words. <span className="text-primary">Get FAIR data.</span>
            </>
          }
          lede="Pair the skills with the MCP server or the CLI, and your agent does the describing and checking."
          actions={
            <>
              <Link to="/mcp-server/" className={actionClass("default")}>
                <Sparkles className="size-4" />
                Connect the MCP server
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

function HeroTerminal() {
  return (
    <pre className="p-5 text-sm leading-relaxed font-mono overflow-x-auto">
      <code className={tk.body}>
        <span className={tk.dim}>$ </span>
        <span className={tk.cmd}>npx</span> skills add{" "}
        <span className={tk.str}>https://fairspec.org</span>
        {"\n\n"}
        <span className={tk.dim}># skills available</span>
        {skills.map(skill => (
          <span key={skill.name}>
            {"\n"}
            <span className={tk.key}>{skill.name}</span>
            {"\n"}
            <span className={tk.dim}>
              {"  "}
              {skill.summary}
            </span>
          </span>
        ))}
      </code>
    </pre>
  )
}

/* ─────────────────────────── Skills ─────────────────────────── */

interface Skill {
  icon: ComponentType<SVGProps<SVGSVGElement>>
  name: string
  summary: string
  when: string
  covers: string[]
}

const skills: Skill[] = [
  {
    icon: Search,
    name: "fairspec-exploration",
    summary: "preview, describe and query tables",
    when: "looking into tabular data: previewing rows, column stats, SQL queries over a CSV, Excel, JSON or Parquet table, a dataset's resources, or a file's source, size and hash.",
    covers: [
      "SQL over one table, or across a whole dataset joined by resource name",
      "First rows, the raw file source and column stats",
      "A dataset's resources and a file's size and hash",
      "Telling dirty values and a misread dialect apart",
    ],
  },
  {
    icon: Wand2,
    name: "fairspec-inference",
    summary: "draft dialects, schemas and datasets",
    when: "describing data files: inferring a file dialect, a table schema, a JSON data schema or a whole dataset descriptor from files.",
    covers: [
      "A dataset descriptor in one pass: dialect, schema and sha256 integrity",
      "A table schema, file dialect or JSON data schema on its own",
      "Hints when the draft is wrong: sample size, comma decimals, pinned column types",
      "Reviewing the draft: keys, required columns and constraints are never inferred",
    ],
  },
  {
    icon: ShieldCheck,
    name: "fairspec-validation",
    summary: "check data and read the report",
    when: "checking whether data, a table, a dataset, a file or a schema conforms to Fairspec, and reading the report's errors to fix them.",
    covers: [
      "One { valid, errors } report for tables, datasets, JSON data, schemas and files",
      "Validating a hand-written schema before the data",
      "Each error type mapped to its usual fix",
      "Errors summarised by type and column, not row by row",
    ],
  },
]

function Skills() {
  return (
    <LandingSection
      title="Three skills, one workflow"
      lede="Explore the data, draft its metadata, then check the data against it. Each skill knows the CLI command, the MCP tool and the library call for every step."
    >
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {skills.map((skill, i) => (
          <Reveal key={skill.name} delayMs={i * 60}>
            <SkillCard {...skill} />
          </Reveal>
        ))}
      </div>
    </LandingSection>
  )
}

function SkillCard(props: Skill) {
  const { icon: Icon, name, when, covers } = props
  return (
    <div className="flex h-full flex-col rounded-card border border-border bg-card p-6">
      <div className="flex min-w-0 items-center gap-3">
        <div className="shrink-0 inline-flex items-center justify-center size-10 rounded-lg bg-primary/10 text-primary">
          <Icon className="size-5" />
        </div>
        <h3 className="min-w-0 truncate font-mono text-base font-semibold text-foreground">
          {name}
        </h3>
      </div>
      <p className="mt-4 text-sm text-muted-foreground leading-relaxed">
        <span className="font-medium text-foreground">Your agent uses it when</span>{" "}
        {when}
      </p>
      <ul className="mt-4 space-y-2 border-t border-border pt-4 text-sm text-foreground">
        {covers.map(item => (
          <li key={item} className="flex gap-2">
            <ListChecks className="mt-0.5 size-4 shrink-0 text-primary" />
            <span>{item}</span>
          </li>
        ))}
      </ul>
    </div>
  )
}

/* ─────────────────────────── Install ─────────────────────────── */

interface InstallStep {
  title: string
  description: ReactNode
  commands: string[]
}

const installSteps: InstallStep[] = [
  {
    title: "From the website",
    description:
      "Run it at your project root. It asks which skills to install and for which agents.",
    commands: ["npx skills add https://fairspec.org"],
  },
  {
    title: "Without prompts",
    description: "Name the skills and the agent up front, for scripts and CI.",
    commands: [
      "npx skills add https://fairspec.org --skill fairspec-validation --agent claude-code",
    ],
  },
  {
    title: "For every project",
    description: (
      <>
        <code className="font-mono text-foreground">--global</code> installs for your user
        instead of the project; <code className="font-mono text-foreground">--list</code>{" "}
        shows what's available without installing anything.
      </>
    ),
    commands: [
      "npx skills add https://fairspec.org --global",
      "npx skills add https://fairspec.org --list",
    ],
  },
  {
    title: "Keep them current",
    description: "Pull the latest versions of the skills you installed.",
    commands: ["npx skills update"],
  },
]

function Install() {
  return (
    <LandingSection
      id="install"
      tinted
      title="Install in one command"
      lede="fairspec.org publishes its skills, so the skills CLI installs them straight from the site."
    >
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-6">
        {installSteps.map((step, i) => (
          <Reveal key={step.title} delayMs={i * 60}>
            <div className="flex h-full min-w-0 flex-col">
              <h3 className="text-lg font-semibold text-foreground">{step.title}</h3>
              <p className="mt-1 mb-3 text-sm text-muted-foreground leading-relaxed">
                {step.description}
              </p>
              <WindowCard label="terminal">
                <pre className="p-4 text-sm leading-relaxed font-mono overflow-x-auto">
                  <code className={tk.body}>
                    {step.commands.map((command, j) => (
                      <span key={command}>
                        {j > 0 && "\n"}
                        <span className={tk.dim}>$ </span>
                        {command}
                      </span>
                    ))}
                  </code>
                </pre>
              </WindowCard>
            </div>
          </Reveal>
        ))}
      </div>
    </LandingSection>
  )
}

/* ─────────────────────────── How it works ─────────────────────────── */

const linkClass = "text-primary underline-offset-4 hover:underline"

const howPoints: Point[] = [
  {
    icon: Bot,
    title: "Loaded when the task matches",
    description:
      "Agents read each skill's description and load it on their own. Ask to check a CSV and the validation skill comes in; no need to name it.",
  },
  {
    icon: SquareTerminal,
    title: "Through the CLI",
    description: (
      <>
        An agent with a shell runs{" "}
        <code className="font-mono text-foreground">fairspec</code> commands with{" "}
        <code className="font-mono text-foreground">--json</code> and reads the results as
        data. See the{" "}
        <Link to="/cli-tool/" className={linkClass}>
          CLI tool
        </Link>
        .
      </>
    ),
  },
  {
    icon: Sparkles,
    title: "Through the MCP server",
    description: (
      <>
        Assistants connected to the{" "}
        <Link to="/mcp-server/" className={linkClass}>
          MCP server
        </Link>{" "}
        call its tools, and the skills tell them which tool fits the question.
      </>
    ),
  },
  {
    icon: Code,
    title: "Through the libraries",
    description: (
      <>
        When the agent writes code, the skills give the matching{" "}
        <Link to="/$" params={{ _splat: "python/getting-started" }} className={linkClass}>
          Python
        </Link>{" "}
        and{" "}
        <Link
          to="/$"
          params={{ _splat: "typescript/getting-started" }}
          className={linkClass}
        >
          TypeScript
        </Link>{" "}
        calls.
      </>
    ),
  },
  {
    icon: Globe,
    title: "Published with the site",
    description: (
      <>
        fairspec.org serves the skills under{" "}
        <code className="font-mono text-foreground">/.well-known/agent-skills/</code>, so
        any agent or tool can discover them.
      </>
    ),
  },
]

function HowItWorks() {
  return (
    <LandingSection
      title="How it works"
      lede="Skills carry the know-how; the CLI, the MCP server and the libraries do the work."
    >
      <div className="grid grid-cols-1 md:grid-cols-2 gap-10 md:gap-12 items-center">
        <PointList points={howPoints} />
        <Reveal>
          <WindowCard label="agent prompt">
            <AgentPrompt />
          </WindowCard>
        </Reveal>
      </div>
      <Reveal>
        <div className="mt-10 text-center md:mt-12">
          <Link to="/$" params={{ _splat: "overview" }} className={linkClass}>
            <BookOpen className="mr-1.5 inline size-4" />
            Read the Fairspec standard →
          </Link>
        </div>
      </Reveal>
    </LandingSection>
  )
}

function AgentPrompt() {
  return (
    <pre className="p-5 text-sm leading-relaxed font-mono overflow-x-auto">
      <code className={tk.body}>
        <span className={tk.key}>{">"}</span> Describe the CSV files in data/ as a{"\n"}
        {"  "}Fairspec dataset, then validate it.
        {"\n\n"}
        <span className={tk.dim}># the agent loads</span>
        {"\n"}
        <span className={tk.str}>fairspec-inference</span>
        {"\n"}
        <span className={tk.dim}>{"  "}fairspec dataset infer data/*.csv --json</span>
        {"\n"}
        <span className={tk.str}>fairspec-validation</span>
        {"\n"}
        <span className={tk.dim}>{"  "}fairspec dataset validate dataset.json</span>
      </code>
    </pre>
  )
}
