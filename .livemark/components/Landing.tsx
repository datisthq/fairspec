import { Link } from "@tanstack/react-router"
import { FileText } from "lucide-react"
import { buttonVariants } from "livemark/elements/button"
import { useInView } from "livemark/hooks/in-view"
import {
  type ComponentType,
  type ReactNode,
  type SVGProps,
  useRef,
  useState,
} from "react"
import { cn } from "cn"

export interface Shot {
  src: string
  srcDark: string
  alt: string
}

export type Slide = {
  id: string
  title: string
  source: { eyebrow: string; file: string; code: ReactNode }
} & ({ shot: Shot } | { content: ReactNode })

export interface InstallCommand {
  command: string
  target: string
}

export interface Point {
  icon: ComponentType<SVGProps<SVGSVGElement>>
  title: string
  description: ReactNode
  link?: { to: string; splat?: string }
}

/** Catppuccin Latte/Mocha token colours for code samples, matching the site's Shiki themes. */
export const tk = {
  punct: "text-[#7c7f93] dark:text-[#9399b2]",
  key: "text-[#1e66f5] dark:text-[#89b4fa]",
  str: "text-[#40a02b] dark:text-[#a6e3a1]",
  num: "text-[#fe640b] dark:text-[#fab387]",
  url: "text-[#179299] dark:text-[#94e2d5]",
  cmd: "text-[#8839ef] dark:text-[#cba6f7]",
  body: "text-[#4c4f69] dark:text-[#cdd6f4]",
  dim: "text-[#9ca0b0] dark:text-[#6c7086]",
}

export type Language = "python" | "typescript" | "rust"

export const languages: { key: Language; name: string }[] = [
  { key: "python", name: "Python" },
  { key: "typescript", name: "TypeScript" },
  { key: "rust", name: "Rust" },
]

export function LanguageTabs(props: {
  language: Language
  onChange: (language: Language) => void
}) {
  return (
    <div
      role="tablist"
      aria-label="Language"
      className="inline-flex gap-1 rounded-control border border-border bg-card p-1"
    >
      {languages.map(l => (
        <button
          key={l.key}
          type="button"
          role="tab"
          aria-selected={l.key === props.language}
          onClick={() => props.onChange(l.key)}
          className={cn(
            "rounded-md px-3 py-1 text-sm font-medium transition-colors",
            l.key === props.language
              ? "bg-primary text-primary-foreground"
              : "text-muted-foreground hover:text-foreground",
          )}
        >
          {l.name}
        </button>
      ))}
    </div>
  )
}

export const textLinkClass =
  "text-sm font-medium text-primary underline-offset-4 hover:underline"

export function actionClass(variant: "default" | "outline" | "ghost") {
  return cn(
    buttonVariants({ variant, size: "lg" }),
    "flex-1 px-3 no-underline sm:flex-none sm:px-5",
  )
}

export function Hero(props: {
  eyebrow: string
  title: ReactNode
  lede: ReactNode
  actions: ReactNode
  install?: InstallCommand[]
  slides: Slide[]
}) {
  return (
    <section className="relative overflow-hidden border-b border-border">
      <HeroBackground />
      <div className="relative mx-auto w-full max-w-6xl px-6 py-12 sm:py-20 md:py-28 lg:pt-8 lg:pb-14 lg:grid lg:grid-cols-[27rem_minmax(0,1fr)] lg:items-center lg:gap-14">
        <div className="animate-in fade-in-0 slide-in-from-bottom-4 duration-700 ease-out motion-reduce:animate-none">
          <p className="text-primary text-xs font-semibold tracking-[0.18em] uppercase">
            {props.eyebrow}
          </p>
          <h1 className="mt-5 text-[2rem] font-semibold tracking-tight text-foreground sm:text-5xl md:text-6xl lg:text-5xl">
            {props.title}
          </h1>
          <p className="mt-6 max-w-[52ch] text-base leading-relaxed text-muted-foreground sm:text-lg md:text-xl">
            {props.lede}
          </p>
          <div className="mt-8 flex flex-wrap gap-3 sm:mt-10">{props.actions}</div>
          {props.install ? <InstallCard commands={props.install} /> : null}
        </div>
        <div className="mt-12 lg:mt-0 xl:w-[60rem] animate-in fade-in-0 slide-in-from-bottom-4 duration-700 ease-out motion-reduce:animate-none">
          <HeroCarousel slides={props.slides} />
        </div>
      </div>
    </section>
  )
}

function InstallCard(props: { commands: InstallCommand[] }) {
  return (
    <ul className="mt-6 hidden w-max list-none gap-1 rounded-card border border-border bg-background/50 px-5 py-3 backdrop-blur-sm sm:grid">
      {props.commands.map(c => (
        <li
          key={c.command}
          className="font-mono text-[0.78125rem] leading-5 whitespace-nowrap text-muted-foreground before:text-primary before:content-['$_'] before:select-none"
        >
          <samp className="font-[inherit]">
            <b className="font-normal text-foreground">{c.command}</b> {c.target}
          </samp>
        </li>
      ))}
    </ul>
  )
}

// Every slide is in the prerendered HTML, stacked in one grid cell; only the active
// one is visible and reachable. All shots share one aspect, so the window never jumps.
function HeroCarousel(props: { slides: Slide[] }) {
  const [active, setActive] = useState(0)
  const [paused, setPaused] = useState(false)
  const [stopped, setStopped] = useState(false)
  // A bar can finish in the same frame as the click that stops it; its end event
  // still holds the old closure, so it checks this instead of `stopped`.
  const stoppedRef = useRef(false)

  return (
    <div
      className="relative"
      onMouseEnter={() => setPaused(true)}
      onMouseLeave={() => setPaused(false)}
      onFocus={() => setPaused(true)}
      onBlur={() => setPaused(false)}
    >
      <figure className="overflow-hidden rounded-card border border-border bg-card shadow-xl">
        <div className="flex items-center gap-5 border-b border-border bg-muted/50 px-4 py-2">
          <span aria-hidden className="hidden gap-1.5 sm:flex">
            <span className="size-2.5 rounded-full bg-border" />
            <span className="size-2.5 rounded-full bg-border" />
            <span className="size-2.5 rounded-full bg-border" />
          </span>
          <div role="tablist" className="flex min-w-0 gap-1">
            {props.slides.map((slide, index) => (
              <button
                key={slide.id}
                type="button"
                role="tab"
                aria-selected={index === active}
                aria-controls={`hero-slide-${slide.id}`}
                onClick={() => {
                  stoppedRef.current = true
                  setActive(index)
                  setStopped(true)
                }}
                className={cn(
                  "relative flex cursor-pointer items-center gap-2 overflow-hidden rounded-control px-3 py-1.5 text-sm whitespace-nowrap transition-colors",
                  index === active
                    ? "bg-card font-medium text-foreground shadow-sm"
                    : "text-muted-foreground hover:text-foreground",
                )}
              >
                <span
                  className={cn("font-mono text-xs", index === active && "text-primary")}
                >
                  {String(index + 1).padStart(2, "0")}
                </span>
                <span className={index === active ? undefined : "hidden sm:inline"}>
                  {slide.title}
                </span>
                {index === active ? (
                  <span
                    key={stopped ? "stopped" : active}
                    aria-hidden
                    className={cn(
                      "absolute inset-x-3 bottom-0.5 h-0.5 rounded-full bg-primary/70",
                      !stopped && "hero-carousel-progress",
                    )}
                    style={{ animationPlayState: paused ? "paused" : "running" }}
                    onAnimationEnd={() => {
                      if (stoppedRef.current) return
                      setActive(current => (current + 1) % props.slides.length)
                    }}
                  />
                ) : null}
              </button>
            ))}
          </div>
        </div>
        <div className="grid">
          {props.slides.map((slide, index) => (
            <div
              key={slide.id}
              id={`hero-slide-${slide.id}`}
              role="tabpanel"
              aria-hidden={index !== active}
              inert={index !== active}
              className={cn(
                "min-w-0 [grid-area:1/1] transition-opacity duration-700 motion-reduce:transition-none",
                index === active ? "opacity-100" : "pointer-events-none opacity-0",
              )}
            >
              {"shot" in slide ? (
                <ShotPicture shot={slide.shot} className="aspect-[16/10] w-full" />
              ) : (
                <div className="w-full overflow-hidden sm:aspect-[16/10]">
                  {slide.content}
                </div>
              )}
            </div>
          ))}
        </div>
      </figure>
      <div className="mt-4 grid grid-cols-1 lg:absolute lg:-bottom-10 lg:-left-10 lg:mt-0 lg:w-[24rem]">
        {props.slides.map((slide, index) => (
          <div
            key={slide.id}
            aria-hidden={index !== active}
            inert={index !== active}
            className={cn(
              "[grid-area:1/1] transition-opacity duration-700 motion-reduce:transition-none",
              index === active ? "opacity-100" : "pointer-events-none opacity-0",
            )}
          >
            <SourceChip source={slide.source} />
          </div>
        ))}
      </div>
    </div>
  )
}

// Two images, not `<picture media>`: the theme is a class on <html>, which a media
// query never sees. A lazy image that is `display: none` is never fetched.
export function ShotPicture(props: { shot: Shot; className?: string }) {
  return (
    <>
      <img
        src={props.shot.src}
        alt={props.shot.alt}
        decoding="async"
        className={cn("block object-cover object-top dark:hidden", props.className)}
      />
      <img
        src={props.shot.srcDark}
        alt={props.shot.alt}
        decoding="async"
        className={cn("hidden object-cover object-top dark:block", props.className)}
      />
    </>
  )
}

export function ShotCard(props: { shot: Shot }) {
  return (
    <div className="overflow-hidden rounded-card border border-primary/20 bg-card">
      <ShotPicture shot={props.shot} className="aspect-[16/10] w-full" />
    </div>
  )
}

function SourceChip(props: { source: Slide["source"] }) {
  return (
    <figure className="flex h-full gap-3 rounded-card border border-primary/25 bg-card p-4 shadow-lg">
      <span className="flex size-8 shrink-0 items-center justify-center rounded-md bg-primary/10 text-primary">
        <FileText className="size-4" />
      </span>
      <div className="min-w-0">
        <p className="text-xs font-semibold tracking-[0.12em] text-primary uppercase">
          {props.source.eyebrow}
        </p>
        <figcaption className="mt-1 font-mono text-xs text-muted-foreground">
          {props.source.file}
        </figcaption>
        <pre className="mt-2 overflow-x-auto font-mono text-sm leading-relaxed">
          <code className={tk.body}>{props.source.code}</code>
        </pre>
      </div>
    </figure>
  )
}

export function Highlight(props: { children: ReactNode }) {
  return (
    <span className="box-decoration-clone bg-linear-to-r from-primary/20 to-primary/20 bg-size-[100%_0.3em] bg-position-[0_88%] bg-no-repeat">
      {props.children}
    </span>
  )
}

export function LandingSection(props: {
  id?: string
  title: ReactNode
  lede: ReactNode
  switcher?: ReactNode
  tinted?: boolean
  children: ReactNode
}) {
  return (
    <section
      id={props.id}
      className={cn(
        "scroll-mt-16 border-b border-border",
        props.tinted && "bg-primary/5",
      )}
    >
      <div className="mx-auto max-w-6xl px-6 py-16 md:py-24">
        <Reveal>
          <div className="text-center max-w-2xl mx-auto mb-10 md:mb-16">
            {props.switcher ? <div className="mb-6">{props.switcher}</div> : null}
            <h2 className="text-3xl md:text-4xl font-bold tracking-tight">
              {props.title}
            </h2>
            <p className="mt-4 text-muted-foreground text-lg">{props.lede}</p>
          </div>
        </Reveal>
        {props.children}
      </div>
    </section>
  )
}

export function FeatureGrid(props: { features: Point[]; columns?: 2 | 3 }) {
  return (
    <div
      className={cn(
        "grid grid-cols-1 md:grid-cols-2 gap-4",
        props.columns !== 2 && "lg:grid-cols-3",
      )}
    >
      {props.features.map((f, i) => (
        <Reveal key={f.title} delayMs={i * 60}>
          <FeatureCard feature={f} />
        </Reveal>
      ))}
    </div>
  )
}

export function SplitSection(props: {
  id?: string
  title: ReactNode
  lede: ReactNode
  switcher?: ReactNode
  points: Point[]
  visual: ReactNode
  reverse?: boolean
  links?: ReactNode
}) {
  const visual = <Reveal>{props.visual}</Reveal>
  return (
    <section id={props.id} className="scroll-mt-16 border-b border-border">
      <div className="mx-auto max-w-6xl px-6 py-16 md:py-24">
        <Reveal>
          <div className="text-center max-w-2xl mx-auto mb-10 md:mb-16">
            {props.switcher ? <div className="mb-6">{props.switcher}</div> : null}
            <h2 className="text-3xl md:text-4xl font-bold tracking-tight">
              {props.title}
            </h2>
            <p className="mt-4 text-muted-foreground text-lg">{props.lede}</p>
          </div>
        </Reveal>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-10 md:gap-12 items-center">
          {props.reverse ? visual : null}
          <PointList points={props.points} />
          {props.reverse ? null : visual}
        </div>
        {props.links ? (
          <Reveal>
            <div className="mt-10 flex flex-wrap justify-center gap-x-8 gap-y-3 md:mt-12">
              {props.links}
            </div>
          </Reveal>
        ) : null}
      </div>
    </section>
  )
}

export function ShowcaseSection(props: {
  title: ReactNode
  lede: ReactNode
  left: { label: string; content: ReactNode }
  right: { label: string; content: ReactNode }
}) {
  return (
    <section className="border-b border-border bg-primary/5">
      <div className="mx-auto max-w-6xl px-6 py-16 md:py-24">
        <Reveal>
          <div className="text-center max-w-2xl mx-auto mb-10 md:mb-16">
            <h2 className="text-3xl md:text-4xl font-bold tracking-tight">
              {props.title}
            </h2>
            <p className="mt-4 text-muted-foreground text-lg">{props.lede}</p>
          </div>
        </Reveal>
        <Reveal>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-6">
            <WindowCard label={props.left.label}>{props.left.content}</WindowCard>
            <WindowCard label={props.right.label}>{props.right.content}</WindowCard>
          </div>
        </Reveal>
      </div>
    </section>
  )
}

export function PointList(props: { points: Point[] }) {
  return (
    <ul className="space-y-8">
      {props.points.map((p, i) => (
        <li key={p.title}>
          <Reveal delayMs={i * 60}>
            <div className="flex gap-4">
              <div className="shrink-0 inline-flex items-center justify-center size-10 rounded-lg bg-primary/10 text-primary">
                <p.icon className="size-5" />
              </div>
              <div className="min-w-0">
                <h3 className="text-lg font-semibold text-foreground">{p.title}</h3>
                <p className="mt-1 text-sm text-muted-foreground leading-relaxed">
                  {p.description}
                </p>
              </div>
            </div>
          </Reveal>
        </li>
      ))}
    </ul>
  )
}

export function WindowCard(props: {
  label: string
  className?: string
  children: ReactNode
}) {
  return (
    <div
      className={cn(
        "rounded-card border border-primary/20 bg-card overflow-hidden text-left",
        props.className,
      )}
    >
      <div className="flex items-center gap-2 border-b border-primary/20 px-4 py-2 bg-muted/50">
        <div className="size-2.5 rounded-full bg-red-400/60" />
        <div className="size-2.5 rounded-full bg-yellow-400/60" />
        <div className="size-2.5 rounded-full bg-green-400/60" />
        <span className="ml-2 truncate text-xs font-mono text-muted-foreground">
          {props.label}
        </span>
      </div>
      {props.children}
    </div>
  )
}

export function FinalCta(props: {
  title: ReactNode
  lede: ReactNode
  actions: ReactNode
}) {
  return (
    <section className="relative overflow-hidden">
      <div className="relative mx-auto max-w-3xl px-6 py-16 text-center md:py-24">
        <Reveal>
          <h2 className="text-3xl md:text-5xl font-extrabold tracking-tight">
            {props.title}
          </h2>
          <p className="mt-6 text-lg text-muted-foreground">{props.lede}</p>
          <div className="mt-8 flex flex-wrap justify-center gap-3 sm:mt-10">
            {props.actions}
          </div>
        </Reveal>
      </div>
    </section>
  )
}

export function Datist() {
  return (
    <section className="border-t border-border bg-muted">
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

export function Reveal(props: { children: ReactNode; delayMs?: number }) {
  const { ref, isVisible } = useInView()
  return (
    <div
      ref={ref}
      style={{ transitionDelay: `${props.delayMs ?? 0}ms` }}
      className={cn(
        "transition-all duration-700 ease-out motion-reduce:transition-none",
        isVisible ? "opacity-100 translate-y-0" : "opacity-0 translate-y-4",
      )}
    >
      {props.children}
    </div>
  )
}

function FeatureCard(props: { feature: Point }) {
  const f = props.feature
  const body = (
    <>
      <div className="inline-flex items-center justify-center size-10 rounded-lg bg-primary/10 text-primary mb-4 group-hover:bg-primary/15 transition-colors">
        <f.icon className="size-5" />
      </div>
      <h3 className="text-lg font-semibold text-foreground">{f.title}</h3>
      <div className="mt-2 text-sm text-muted-foreground leading-relaxed">
        {f.description}
      </div>
    </>
  )
  const className =
    "block h-full group relative rounded-card border border-border bg-card p-6 no-underline transition-all duration-200 hover:border-primary/40 hover:-translate-y-0.5 hover:shadow-lg motion-reduce:transition-none motion-reduce:hover:translate-y-0"
  if (!f.link) return <div className={className}>{body}</div>
  if (f.link.splat !== undefined) {
    return (
      <Link to="/$" params={{ _splat: f.link.splat }} className={className}>
        {body}
      </Link>
    )
  }
  return (
    <Link to={f.link.to} className={className}>
      {body}
    </Link>
  )
}

function HeroBackground() {
  return (
    <>
      <style href="fairspec-hero-background" precedence="default">
        {HERO_BACKGROUND_CSS}
      </style>
      <div aria-hidden className="pointer-events-none absolute inset-0 overflow-hidden">
        <div className="absolute inset-0 [mask-image:linear-gradient(to_bottom,black_55%,transparent_98%)]">
          <div className="strata strata-drift absolute inset-y-0 -inset-x-11 opacity-50" />
          <div className="hero-scan-x from-primary/0 via-primary/16 to-primary/0 absolute inset-y-0 w-[40%] bg-gradient-to-r" />
          <div className="hero-scan-y from-secondary/0 via-secondary/16 to-secondary/0 absolute inset-x-0 h-[40%] bg-gradient-to-b" />
        </div>
      </div>
      <div
        aria-hidden
        className="hero-drift bg-primary/15 pointer-events-none absolute -top-40 -right-40 size-[32rem] rounded-full blur-3xl"
      />
    </>
  )
}

const HERO_BACKGROUND_CSS = `
@keyframes hero-scan-x {
  0% {
    transform: translateX(-55%);
    opacity: 0;
  }
  5%,
  45% {
    opacity: 1;
  }
  50%,
  100% {
    transform: translateX(155%);
    opacity: 0;
  }
}

@keyframes hero-scan-y {
  0%,
  50% {
    transform: translateY(-55%);
    opacity: 0;
  }
  55%,
  95% {
    opacity: 1;
  }
  100% {
    transform: translateY(155%);
    opacity: 0;
  }
}

@keyframes strata-drift {
  to {
    transform: translateX(44px);
  }
}

@keyframes hero-drift {
  0%,
  100% {
    transform: translate3d(0, 0, 0) scale(1);
  }
  50% {
    transform: translate3d(6%, -8%, 0) scale(1.08);
  }
}

.strata {
  background-image: repeating-linear-gradient(
    90deg,
    var(--color-border) 0,
    var(--color-border) 1px,
    transparent 1px,
    transparent 44px
  );
}

.strata-drift {
  animation: strata-drift 14s linear infinite;
}

.hero-scan-x {
  animation: hero-scan-x 18s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

.hero-scan-y {
  animation: hero-scan-y 18s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

.hero-drift {
  animation: hero-drift 18s ease-in-out infinite;
}

@keyframes hero-carousel-progress {
  from {
    transform: scaleX(0);
  }
  to {
    transform: scaleX(1);
  }
}

.hero-carousel-progress {
  transform-origin: left;
  animation: hero-carousel-progress 6s linear;
}

@media (prefers-reduced-motion: reduce) {
  .hero-scan-x,
  .hero-scan-y {
    animation: none;
    opacity: 0;
  }
  .strata-drift,
  .hero-drift,
  .hero-carousel-progress {
    animation: none;
  }
}
`
