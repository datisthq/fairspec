import { createFileRoute } from "@tanstack/react-router"
import {
  ArrowRight,
  CheckCircle2,
  ExternalLink,
  ListOrdered,
  Search,
  Sparkles,
} from "lucide-react"
import type { ComponentType, ReactNode, SVGProps } from "react"
import { useState } from "react"
import { buttonVariants } from "livemark/elements/button"
import { useInView } from "livemark/hooks/in-view"
import { cn } from "livemark/utils/style"

// @ts-ignore
export const Route = createFileRoute("/application/")({
  component: ApplicationPage,
  head: () => ({
    meta: [
      { title: "Fairspec Application" },
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
    <div className="flex flex-col">
      <Hero />
      <Tools />
      <Downloads />
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
              Fairspec Application
            </h1>
            <p className="mt-6 text-lg md:text-xl text-muted-foreground">
              Visual tool for validating and inferring{" "}
              <strong className="text-foreground">Fairspec</strong> descriptors. Point it
              at your data and get a report — no code required.
            </p>
            <div className="mt-10 flex flex-col sm:flex-row items-center justify-center lg:justify-start gap-3">
              <a
                href="#download"
                className={cn(
                  buttonVariants({ variant: "default", size: "lg" }),
                  "px-5 no-underline",
                )}
              >
                Download
                <ArrowRight className="size-4" />
              </a>
              <a
                href="https://github.com/datisthq/fairspec-application"
                target="_blank"
                rel="noopener noreferrer"
                className={cn(
                  buttonVariants({ variant: "outline", size: "lg" }),
                  "px-5 no-underline",
                )}
              >
                View on GitHub
                <ExternalLink className="size-4" />
              </a>
            </div>
          </div>
          <div className="rounded-xl border border-primary/20 bg-card overflow-hidden">
            <div className="flex items-center gap-2 border-b border-primary/20 px-4 py-2 bg-muted/50">
              <div className="size-2.5 rounded-full bg-red-400/60" />
              <div className="size-2.5 rounded-full bg-yellow-400/60" />
              <div className="size-2.5 rounded-full bg-green-400/60" />
              <span className="ml-2 text-xs font-mono text-muted-foreground">
                Fairspec Application
              </span>
            </div>
            <img
              src="/application.webp"
              width={1200}
              height={750}
              alt="The Fairspec Application with dataset, table, data and file tools in the sidebar"
              className="block w-full"
            />
          </div>
        </div>
      </div>
    </section>
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

/* ─────────────────────────── Tools ─────────────────────────── */

interface Tool {
  icon: ComponentType<SVGProps<SVGSVGElement>>
  title: string
  description: string
}

const tools: Tool[] = [
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
      "Loads a tabular file and shows the first hundred rows alongside the table schema inferred from them, so you can inspect the contents and the detected structure side by side.",
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
    <section className="border-b border-border">
      <div className="mx-auto max-w-6xl px-6 py-24">
        <Reveal>
          <div className="text-center max-w-2xl mx-auto mb-16">
            <h2 className="text-3xl md:text-4xl font-bold tracking-tight">Tools</h2>
            <p className="mt-4 text-muted-foreground text-lg">
              Nine tools across the four Fairspec surfaces the application covers:
              datasets, tables, data, and files.
            </p>
          </div>
        </Reveal>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {tools.map((t, i) => (
            <Reveal key={t.title} delayMs={i * 40}>
              <ToolCard {...t} />
            </Reveal>
          ))}
        </div>
      </div>
    </section>
  )
}

function ToolCard({ icon: Icon, title, description }: Tool) {
  return (
    <div className="block h-full relative rounded-xl border border-border bg-card p-6">
      <div className="flex items-center gap-3 mb-3">
        <div className="inline-flex items-center justify-center size-10 rounded-lg bg-primary/10 text-primary">
          <Icon className="size-5" />
        </div>
        <h3 className="text-lg font-semibold text-foreground">{title}</h3>
      </div>
      <p className="text-sm text-muted-foreground leading-relaxed">{description}</p>
    </div>
  )
}

/* ─────────────────────────── Download ─────────────────────────── */

const RELEASES = "https://github.com/datisthq/fairspec-application/releases"

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
    <section id="download" className="border-b border-border scroll-mt-16">
      <div className="mx-auto max-w-5xl px-6 py-24">
        <Reveal>
          <div className="text-center max-w-2xl mx-auto mb-12">
            <h2 className="text-3xl md:text-4xl font-bold tracking-tight">Download</h2>
            <p className="mt-4 text-muted-foreground text-lg">
              Install the desktop application. All nine tools run locally, on your own
              machine.
            </p>
          </div>
        </Reveal>
        <Reveal>
          <div className="rounded-xl border border-border bg-card overflow-hidden">
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
              <a
                href={platform.action.href}
                target="_blank"
                rel="noopener noreferrer"
                className={cn(
                  buttonVariants({ variant: "default", size: "lg" }),
                  "px-5 no-underline",
                )}
              >
                {platform.action.label}
                <ExternalLink className="size-4" />
              </a>
            </div>
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
