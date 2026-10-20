import { createFileRoute } from "@tanstack/react-router"
import {
  ArrowRight,
  CheckCircle2,
  ExternalLink,
  Github,
  History,
  ListOrdered,
  Search,
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
  Hero,
  LandingSection,
  type Point,
  Reveal,
  WindowCard,
  actionClass,
} from "../components/Landing.tsx"

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
                href="https://github.com/datisthq/fairspec/blob/main/desktop/CHANGELOG.md"
                target="_blank"
                rel="noopener noreferrer"
                className={actionClass("outline")}
              >
                <History className="size-4" />
                Changelog
              </a>
            </>
          }
          visual={
            <WindowCard label="Fairspec Application">
              <img
                src="/application.webp"
                width={1200}
                height={750}
                alt="The Fairspec Application with dataset, table, data and file tools in the sidebar"
                className="block w-full h-auto"
              />
            </WindowCard>
          }
        />
        <Tools />
        <Downloads />
        <Datist />
      </main>
      <BackToTop />
    </>
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

function Tools() {
  return (
    <LandingSection
      title="Tools"
      lede="Nine tools across the four Fairspec surfaces the application covers: datasets, tables, data, and files."
    >
      <FeatureGrid features={tools} />
    </LandingSection>
  )
}

/* ─────────────────────────── Download ─────────────────────────── */

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
    <LandingSection
      id="download"
      tinted
      title="Download"
      lede="Install the desktop application. All nine tools run locally, on your own machine."
    >
      <Reveal>
        <div className="mx-auto max-w-3xl rounded-card border border-border bg-card overflow-hidden">
          <div
            role="tablist"
            className="flex flex-wrap border-b border-border bg-muted/40"
          >
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
        </div>
      </Reveal>
    </LandingSection>
  )
}
