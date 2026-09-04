import { Link, createFileRoute } from "@tanstack/react-router"
import {
  ArrowRight,
  BookOpen,
  Code,
  CodeXml,
  FileText,
  Github,
  Globe,
  Laptop,
  Puzzle,
  Rocket,
  Sparkles,
} from "lucide-react"
import type { ComponentType, ReactNode, SVGProps } from "react"
import { buttonVariants } from "livemark/elements/button"
import { useInView } from "livemark/hooks/in-view"
import { cn } from "livemark/utils/style"

// @ts-ignore
export const Route = createFileRoute("/")({
  component: Landing,
})

function Landing() {
  return (
    <div className="flex flex-col">
      <Hero />
      <Features />
      <Showcase />
      <Implementations />
      <FinalCta />
      <Datist />
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
              <Sparkles className="size-3.5 text-primary" />
              Technical preview
            </div>
            <h1 className="text-5xl md:text-6xl font-extrabold tracking-tight text-foreground">
              Make your data{" "}
              <span className="relative inline-block">
                <span className="relative z-10">FAIR</span>
                <span
                  aria-hidden
                  className="absolute left-0 right-0 bottom-1 md:bottom-2 h-3 md:h-4 bg-primary/20 -z-0 rounded"
                />
              </span>
            </h1>

            <p className="mt-6 text-lg md:text-xl text-muted-foreground">
              Data exchange format compatible with{" "}
              <code className="font-mono text-foreground">DataCite</code> for metadata and{" "}
              <code className="font-mono text-foreground">JSON Schema</code> for
              structured data, shipped with{" "}
              <code className="font-mono text-foreground">Python</code>,{" "}
              <code className="font-mono text-foreground">TypeScript</code>,{" "}
              <code className="font-mono text-foreground">MCP Server</code>, and{" "}
              <code className="font-mono text-foreground">Desktop</code> implementations.
            </p>

            <div className="mt-10 flex flex-col sm:flex-row items-center justify-center lg:justify-start gap-3">
              <Link
                to="/overview/"
                className={cn(
                  buttonVariants({ variant: "default", size: "lg" }),
                  "px-5 no-underline",
                )}
              >
                Get started
                <ArrowRight className="size-4" />
              </Link>
              <a
                href="https://github.com/datisthq/fairspec"
                target="_blank"
                rel="noopener noreferrer"
                className={cn(
                  buttonVariants({ variant: "outline", size: "lg" }),
                  "px-5 no-underline",
                )}
              >
                <Github className="size-4" />
                View source
              </a>
            </div>
          </div>
          <SpecPreview />
        </div>
      </div>
    </section>
  )
}

function SpecPreview() {
  return (
    <div className="rounded-xl border border-primary/20 bg-card overflow-hidden text-left">
      <div className="flex items-center gap-2 border-b border-primary/20 px-4 py-2 bg-muted/50">
        <div className="size-2.5 rounded-full bg-red-400/60" />
        <div className="size-2.5 rounded-full bg-yellow-400/60" />
        <div className="size-2.5 rounded-full bg-green-400/60" />
        <span className="ml-2 text-xs font-mono text-muted-foreground">dataset.json</span>
      </div>
      <div className="p-5 space-y-3 font-mono text-sm leading-relaxed overflow-x-auto">
        <SpecLabel>DataCite compatible metadata</SpecLabel>
        <pre className="overflow-x-auto">
          <DataciteLines />
        </pre>
        <SpecLabel>JSONSchema compatible data</SpecLabel>
        <pre className="overflow-x-auto">
          <TableSchemaLines />
        </pre>
      </div>
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

/* ─────────────────────────── Features ─────────────────────────── */

interface Feature {
  icon: ComponentType<SVGProps<SVGSVGElement>>
  title: string
  description: string
}

const features: Feature[] = [
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
      "Python and TypeScript implementations out-of-the-box, plus an MCP server so AI assistants can validate and query Fairspec data.",
  },
]

function Features() {
  return (
    <section className="border-b border-border">
      <div className="mx-auto max-w-6xl px-6 py-24">
        <Reveal>
          <div className="text-center max-w-2xl mx-auto mb-16">
            <h2 className="text-3xl md:text-4xl font-bold tracking-tight">
              A simple format for FAIR data
            </h2>
            <p className="mt-4 text-muted-foreground text-lg">
              Built on widely-adopted open standards. Easy to adopt, easy to extend.
            </p>
          </div>
        </Reveal>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {features.map((f, i) => (
            <Reveal key={f.title} delayMs={i * 60}>
              <FeatureCard {...f} />
            </Reveal>
          ))}
        </div>
      </div>
    </section>
  )
}

function FeatureCard({ icon: Icon, title, description }: Feature) {
  return (
    <div className="h-full group relative rounded-xl border border-border bg-card p-6 transition-all duration-200 hover:border-primary/40 hover:-translate-y-0.5 hover:shadow-lg">
      <div className="inline-flex items-center justify-center size-10 rounded-lg bg-primary/10 text-primary mb-4 group-hover:bg-primary/15 transition-colors">
        <Icon className="size-5" />
      </div>
      <h3 className="text-lg font-semibold text-foreground">{title}</h3>
      <p className="mt-2 text-sm text-muted-foreground leading-relaxed">{description}</p>
    </div>
  )
}

/* ─────────────────────────── Showcase ─────────────────────────── */

const tk = {
  punct: "text-[#7c7f93] dark:text-[#9399b2]",
  key: "text-[#1e66f5] dark:text-[#89b4fa]",
  str: "text-[#40a02b] dark:text-[#a6e3a1]",
  num: "text-[#fe640b] dark:text-[#fab387]",
  url: "text-[#179299] dark:text-[#94e2d5]",
  body: "text-[#4c4f69] dark:text-[#cdd6f4]",
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

function Showcase() {
  return (
    <section className="border-b border-border bg-primary/5">
      <div className="mx-auto max-w-6xl px-6 py-24">
        <Reveal>
          <div className="text-center max-w-2xl mx-auto mb-16">
            <h2 className="text-3xl md:text-4xl font-bold tracking-tight">
              One JSON file. A FAIR dataset.
            </h2>
            <p className="mt-4 text-muted-foreground text-lg">
              A Fairspec descriptor carries everything a consumer needs: citation
              metadata, file dialects, and validated schemas.
            </p>
          </div>
        </Reveal>
        <Reveal>
          <div className="rounded-xl border border-primary/20 bg-card overflow-hidden max-w-4xl mx-auto">
            <div className="flex items-center gap-2 border-b border-primary/20 px-4 py-2 bg-muted/50">
              <div className="size-2.5 rounded-full bg-red-400/60" />
              <div className="size-2.5 rounded-full bg-yellow-400/60" />
              <div className="size-2.5 rounded-full bg-green-400/60" />
              <span className="ml-2 text-xs font-mono text-muted-foreground">
                dataset.json
              </span>
            </div>
            <DatasetSample />
          </div>
        </Reveal>
      </div>
    </section>
  )
}

/* ─────────────────────────── Implementations ─────────────────────────── */

interface Implementation {
  icon: ComponentType<SVGProps<SVGSVGElement>>
  title: string
  description: string
  command: string
  href: string
  external: boolean
}

const implementations: Implementation[] = [
  {
    icon: Code,
    title: "Python",
    description:
      "Validate, infer, and query Fairspec descriptors from Python, with first-class support for pandas and Polars DataFrames.",
    command: "pip install fairspec",
    href: "https://python.fairspec.org",
    external: true,
  },
  {
    icon: CodeXml,
    title: "TypeScript",
    description:
      "The same API surface as the Python package, built on Polars, and shipping a terminal CLI alongside the library.",
    command: "npm install fairspec",
    href: "https://typescript.fairspec.org",
    external: true,
  },
  {
    icon: Sparkles,
    title: "MCP Server",
    description:
      "Connects AI assistants to the framework over the Model Context Protocol, so you can validate and infer in plain language.",
    command: "npx fairspec@latest mcp",
    href: "/mcp-server/",
    external: false,
  },
  {
    icon: Laptop,
    title: "Application",
    description:
      "A desktop application for validating and inferring descriptors through a visual interface, with your files never leaving your machine.",
    command: "Download for macOS, Windows, Linux",
    href: "/application/",
    external: false,
  },
]

function Implementations() {
  return (
    <section className="border-b border-border">
      <div className="mx-auto max-w-6xl px-6 py-24">
        <Reveal>
          <div className="text-center max-w-2xl mx-auto mb-16">
            <h2 className="text-3xl md:text-4xl font-bold tracking-tight">
              Implementations
            </h2>
            <p className="mt-4 text-muted-foreground text-lg">
              Adopt the standard from your language of choice, your AI assistant, or a
              desktop interface.
            </p>
          </div>
        </Reveal>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {implementations.map((impl, i) => (
            <Reveal key={impl.title} delayMs={i * 60}>
              <ImplementationCard {...impl} />
            </Reveal>
          ))}
        </div>
      </div>
    </section>
  )
}

function ImplementationCard(props: Implementation) {
  const { icon: Icon, title, description, command, href, external } = props
  const className =
    "group flex flex-col h-full rounded-xl border border-border bg-card p-6 no-underline transition-all hover:border-primary/40 hover:-translate-y-0.5 hover:shadow-lg"
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
      <div className="mt-4 rounded-md border border-border bg-background px-3 py-2 font-mono text-xs text-muted-foreground">
        {command}
      </div>
    </>
  )
  if (external) {
    return (
      <a href={href} target="_blank" rel="noopener noreferrer" className={className}>
        {body}
      </a>
    )
  }
  return (
    <Link to={href} className={className}>
      {body}
    </Link>
  )
}

/* ─────────────────────────── Final CTA ─────────────────────────── */

function FinalCta() {
  return (
    <section className="relative overflow-hidden">
      <div className="relative mx-auto max-w-3xl px-6 py-24 text-center">
        <Reveal>
          <h2 className="text-3xl md:text-5xl font-extrabold tracking-tight">
            Describe your data. <span className="text-primary">Make it FAIR.</span>
          </h2>
          <p className="mt-6 text-lg text-muted-foreground">
            Read the specifications, browse the examples, and start describing your
            datasets in minutes.
          </p>
          <div className="mt-10 flex flex-col sm:flex-row items-center justify-center gap-3">
            <Link
              to="/overview/"
              className={cn(
                buttonVariants({ variant: "default", size: "lg" }),
                "px-5 no-underline",
              )}
            >
              Read the specs
              <ArrowRight className="size-4" />
            </Link>
            <Link
              to="/mcp-server/"
              className={cn(
                buttonVariants({ variant: "ghost", size: "lg" }),
                "px-5 no-underline",
              )}
            >
              <Sparkles className="size-4" />
              Try the MCP server
            </Link>
            <Link
              to="/application/"
              className={cn(
                buttonVariants({ variant: "ghost", size: "lg" }),
                "px-5 no-underline",
              )}
            >
              <Laptop className="size-4" />
              Open the application
            </Link>
          </div>
        </Reveal>
      </div>
    </section>
  )
}

/* ─────────────────────────── Datist ─────────────────────────── */

function Datist() {
  return (
    <section className="border-t border-border">
      <div className="mx-auto max-w-5xl px-6 py-10">
        <Reveal>
          <div className="flex flex-col items-center gap-3 text-center">
            <p className="text-xs font-semibold uppercase tracking-widest text-muted-foreground">
              Built by
            </p>
            <a
              href="https://datist.io"
              target="_blank"
              rel="noopener"
              className="text-sm font-medium text-foreground/70 hover:text-foreground transition-colors no-underline"
            >
              Datist — Software development, end to end.
            </a>
          </div>
        </Reveal>
      </div>
    </section>
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
