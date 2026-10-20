import { buttonVariants } from "livemark/elements/button"
import { useInView } from "livemark/hooks/in-view"
import type { ComponentType, ReactNode, SVGProps } from "react"
import { cn } from "cn"

export interface Point {
  icon: ComponentType<SVGProps<SVGSVGElement>>
  title: string
  description: ReactNode
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
  visual: ReactNode
}) {
  return (
    <section className="relative overflow-hidden border-b border-border">
      <HeroBackground />
      <div className="relative mx-auto w-full max-w-6xl px-6 py-12 sm:py-20 md:py-28 lg:grid lg:grid-cols-[27rem_minmax(0,1fr)] lg:items-center lg:gap-14">
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
        </div>
        <div className="mt-12 min-w-0 lg:mt-0 animate-in fade-in-0 slide-in-from-bottom-4 duration-700 ease-out motion-reduce:animate-none">
          {props.visual}
        </div>
      </div>
    </section>
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

export function FeatureGrid(props: { features: Point[] }) {
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {props.features.map((f, i) => (
        <Reveal key={f.title} delayMs={i * 60}>
          <div className="h-full group relative rounded-card border border-border bg-card p-6 transition-all duration-200 hover:border-primary/40 hover:-translate-y-0.5 hover:shadow-lg motion-reduce:transition-none motion-reduce:hover:translate-y-0">
            <div className="inline-flex items-center justify-center size-10 rounded-lg bg-primary/10 text-primary mb-4 group-hover:bg-primary/15 transition-colors">
              <f.icon className="size-5" />
            </div>
            <h3 className="text-lg font-semibold text-foreground">{f.title}</h3>
            <p className="mt-2 text-sm text-muted-foreground leading-relaxed">
              {f.description}
            </p>
          </div>
        </Reveal>
      ))}
    </div>
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

@media (prefers-reduced-motion: reduce) {
  .hero-scan-x,
  .hero-scan-y {
    animation: none;
    opacity: 0;
  }
  .strata-drift,
  .hero-drift {
    animation: none;
  }
}
`
