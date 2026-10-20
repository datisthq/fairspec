import { Link, createFileRoute } from "@tanstack/react-router"
import {
  ArrowRight,
  CheckCircle2,
  Code,
  CodeXml,
  ExternalLink,
  Github,
  History,
  ListOrdered,
  Monitor,
  MousePointerClick,
  Search,
  ShieldCheck,
  Sparkles,
  SquareTerminal,
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
  ShowcaseSection,
  type Slide,
  SplitSection,
  WindowCard,
  actionClass,
  textLinkClass,
  ShotCard,
} from "../components/Landing.tsx"
import { BrokenCsv, shots, ValidationErrors, WindowCode } from "../components/Panes.tsx"

// Livemark generates its route tree at build time, so this route id
// is not known to the checker here.
// @ts-ignore
export const Route = createFileRoute("/desktop/")({
  component: ApplicationPage,
  head: () => ({
    meta: [
      { title: "Fairspec Desktop" },
      {
        name: "description",
        content:
          "Visual tool for validating and inferring Fairspec descriptors for tabular and structured data.",
      },
    ],
  }),
})

const CHANGELOG = "https://github.com/datisthq/fairspec/blob/main/desktop/CHANGELOG.md"

function ApplicationPage() {
  return (
    <>
      <Banner />
      <Header />
      <main className="flex flex-col">
        <Hero
          eyebrow="Technical preview"
          title="Fairspec Desktop"
          lede={
            <>
              Visual tool for validating and inferring{" "}
              <strong className="text-foreground">Fairspec</strong> descriptors. Point it
              at your data and get a report — no code required.
            </>
          }
          actions={
            <>
              <a href="#download" className={actionClass("default")}>
                Download
                <ArrowRight className="size-4" />
              </a>
              <a
                href="https://github.com/datisthq/fairspec/tree/main/desktop"
                target="_blank"
                rel="noopener noreferrer"
                className={actionClass("outline")}
              >
                <Github className="size-4" />
                View on GitHub
              </a>
              <a
                href={CHANGELOG}
                target="_blank"
                rel="noopener noreferrer"
                className={actionClass("outline")}
              >
                <History className="size-4" />
                Changelog
              </a>
            </>
          }
          slides={slides}
        />
        <SplitSection
          id="download"
          title="Private by design"
          lede="Install the desktop application. All nine tools run locally, on your own machine."
          points={privacyPoints}
          visual={<Downloads />}
        />
        <LandingSection
          title="Tools"
          lede="Nine tools across the four Fairspec surfaces the application covers: datasets, tables, data, and files."
        >
          <FeatureGrid features={tools} />
        </LandingSection>
        <ShowcaseSection
          title="The same report, without code"
          lede="Open a file, pick a tool, and read the problems the way the CLI and the libraries report them."
          left={{
            label: "broken.csv",
            content: (
              <WindowCode>
                <BrokenCsv />
              </WindowCode>
            ),
          }}
          right={{
            label: "Validate Table",
            content: (
              <WindowCode>
                <ValidationErrors />
              </WindowCode>
            ),
          }}
        />
        <SplitSection
          reverse
          title="Prefer code?"
          lede="The checks the application runs are also commands, library calls and, for most, MCP tools."
          points={codePoints}
          visual={<ShotCard shot={shots.validate} />}
          links={
            <>
              <Link to="/cli-tool/" className={textLinkClass}>
                Use the CLI tool →
              </Link>
              <Link
                to="/$"
                params={{ _splat: "python/getting-started" }}
                className={textLinkClass}
              >
                Python library →
              </Link>
              <Link
                to="/$"
                params={{ _splat: "typescript/getting-started" }}
                className={textLinkClass}
              >
                TypeScript library →
              </Link>
            </>
          }
        />
        <FinalCta
          title={
            <>
              Point it at your data. <span className="text-primary">Get a report.</span>
            </>
          }
          lede="Download the application for macOS, Windows or Linux, or see what changed in the latest release."
          actions={
            <>
              <a href="#download" className={actionClass("default")}>
                Download
                <ArrowRight className="size-4" />
              </a>
              <a
                href={CHANGELOG}
                target="_blank"
                rel="noopener noreferrer"
                className={actionClass("ghost")}
              >
                <History className="size-4" />
                Changelog
              </a>
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

const slides: Slide[] = [
  {
    id: "app",
    title: "Application",
    shot: shots.desktop,
    source: {
      eyebrow: "No code required",
      file: "macOS · Windows · Linux",
      code: <>Your files never leave your machine.</>,
    },
  },
]

/* ─────────────────────────── Download ─────────────────────────── */

const privacyPoints: Point[] = [
  {
    icon: ShieldCheck,
    title: "Your files stay local",
    description:
      "Every tool runs on your own machine. Your files never leave your machine.",
  },
  {
    icon: MousePointerClick,
    title: "No code required",
    description: "Point it at your data and get a report.",
  },
  {
    icon: Monitor,
    title: "macOS, Windows and Linux",
    description: (
      <>
        Installers (<code className="font-mono text-foreground">.dmg</code>,{" "}
        <code className="font-mono text-foreground">.exe</code>,{" "}
        <code className="font-mono text-foreground">.AppImage</code>,{" "}
        <code className="font-mono text-foreground">.deb</code>) on the GitHub releases
        page.
      </>
    ),
  },
  {
    icon: CodeXml,
    title: "Built on the TypeScript library",
    description: (
      <>
        Runs <code className="font-mono text-foreground">@fairspec/library</code>, the
        engine behind the TypeScript package and CLI.
      </>
    ),
  },
]

const RELEASES = "https://github.com/datisthq/fairspec/releases?q=desktop"

interface Platform {
  name: string
  description: ReactNode
  action: { label: string; href: string }
}

const platforms: Platform[] = [
  {
    name: "macOS",
    description: (
      <>
        Download the <code className="font-mono text-foreground">.dmg</code> installer
        from the latest release and drag the application into your Applications folder.
        Your files never leave your machine.
      </>
    ),
    action: { label: "Download for macOS", href: RELEASES },
  },
  {
    name: "Windows",
    description: (
      <>
        Download the <code className="font-mono text-foreground">.exe</code> installer
        from the latest release and run it. Your files never leave your machine.
      </>
    ),
    action: { label: "Download for Windows", href: RELEASES },
  },
  {
    name: "Linux",
    description: (
      <>
        Download the <code className="font-mono text-foreground">.AppImage</code> or{" "}
        <code className="font-mono text-foreground">.deb</code> package from the latest
        release and run it. Your files never leave your machine.
      </>
    ),
    action: { label: "Download for Linux", href: RELEASES },
  },
]

function Downloads() {
  const [active, setActive] = useState(0)
  const platform = platforms[active] ?? platforms[0]
  if (!platform) return null
  return (
    <WindowCard label="Download">
      <div role="tablist" className="flex flex-wrap border-b border-border bg-muted/40">
        {platforms.map((p, i) => (
          <button
            key={p.name}
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
            {p.name}
          </button>
        ))}
      </div>
      <div className="p-6">
        <p className="text-sm text-muted-foreground leading-relaxed mb-6">
          {platform.description}
        </p>
        <div className="flex flex-wrap gap-3">
          <a
            href={platform.action.href}
            target="_blank"
            rel="noopener noreferrer"
            className={actionClass("default")}
          >
            {platform.action.label}
            <ExternalLink className="size-4" />
          </a>
        </div>
      </div>
    </WindowCard>
  )
}

/* ─────────────────────────── Tools ─────────────────────────── */

const tools: Point[] = [
  {
    icon: CheckCircle2,
    title: "Validate Dataset",
    description:
      "Checks a dataset descriptor against the Fairspec specification and verifies that every resource it references is consistent. Reports each violation with the exact location in the descriptor.",
  },
  {
    icon: Search,
    title: "Infer Dataset",
    description:
      "Scans a folder of data files and builds a complete dataset descriptor from what it finds, discovering tabular resources and inferring their schemas and dialects along the way.",
  },
  {
    icon: ListOrdered,
    title: "Preview Table",
    description:
      "Loads a tabular file and shows the start of the data alongside the table schema inferred from it, so you can inspect the contents and the detected structure side by side.",
  },
  {
    icon: CheckCircle2,
    title: "Validate Table",
    description:
      "Checks tabular data against a table schema, optionally with an explicit file dialect. Groups the failures by error type so you can see at a glance which problems dominate.",
  },
  {
    icon: Search,
    title: "Infer Table Schema",
    description:
      "Reads a tabular file and derives a table schema with field names, types, and constraints, analysing the values in each column to pick the most appropriate type.",
  },
  {
    icon: CheckCircle2,
    title: "Validate Data",
    description:
      "Checks structured data against a data schema, flagging type mismatches, constraint violations, missing required values, and broken key relationships.",
  },
  {
    icon: Search,
    title: "Infer Data Schema",
    description:
      "Derives a JSON Schema from a data sample, examining the structure and values to describe the types and constraints that the data actually satisfies.",
  },
  {
    icon: Search,
    title: "Infer File Dialect",
    description:
      "Detects how a tabular file is formatted — delimiter, quote character, header row, line terminator, and encoding — by sampling its contents.",
  },
  {
    icon: CheckCircle2,
    title: "Validate File",
    description:
      "Verifies a file against an expected checksum, supporting md5, sha1, sha256, and sha512, so you can confirm that the bytes you received are the bytes that were published.",
  },
]

/* ─────────────────────────── Code ─────────────────────────── */

const codePoints: Point[] = [
  {
    icon: SquareTerminal,
    title: "CLI tool",
    description:
      "Validate datasets, infer schemas, and query tables from scripts, CI pipelines, or an interactive terminal.",
  },
  {
    icon: Code,
    title: "Python",
    description:
      "Validate, infer, and query Fairspec descriptors from Python, with first-class support for pandas and Polars DataFrames.",
  },
  {
    icon: CodeXml,
    title: "TypeScript",
    description:
      "The same API surface as the Python package, built on Polars, and shipping a terminal CLI alongside the library.",
  },
  {
    icon: Sparkles,
    title: "MCP server",
    description:
      "Connects AI assistants to the framework over the Model Context Protocol, so you can validate and infer in plain language.",
  },
]
