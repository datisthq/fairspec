import { Trans, useLingui } from "@lingui/react/macro"
import { ChevronDown } from "lucide-react"
import { useEffect, useRef, useState } from "react"
import type { HelpEntry } from "#helpers/help.ts"
import * as icons from "#icons.ts"

export function Help(props: { entry?: HelpEntry }) {
  if (!props.entry) return null

  return (
    <aside className="hidden xl:block w-56 shrink-0">
      <div className="sticky top-20 max-h-[calc(100vh-6rem)] overflow-auto [scrollbar-width:none] border-s ps-4">
        <HelpBody entry={props.entry} />
      </div>
    </aside>
  )
}

export function MobileHelp(props: { entry?: HelpEntry }) {
  const { t } = useLingui()
  const [open, setOpen] = useState(false)
  const wrapperRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (!open) return

    const handlePointerDown = (event: PointerEvent) => {
      if (event.target instanceof Node && wrapperRef.current?.contains(event.target)) {
        return
      }

      setOpen(false)
    }

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") setOpen(false)
    }

    document.addEventListener("pointerdown", handlePointerDown)
    document.addEventListener("keydown", handleKeyDown)

    return () => {
      document.removeEventListener("pointerdown", handlePointerDown)
      document.removeEventListener("keydown", handleKeyDown)
    }
  }, [open])

  if (!props.entry) return null

  return (
    <div ref={wrapperRef} className="xl:hidden sticky top-16 z-20 border-b bg-background">
      <button
        type="button"
        onClick={() => setOpen(value => !value)}
        aria-expanded={open}
        className="flex w-full items-center gap-3 px-4 py-3 text-sm text-muted-foreground"
      >
        <icons.Help className="size-4" />
        <span className="truncate flex-1 text-left">{t`Help`}</span>
        <ChevronDown
          className={`size-4 transition-transform ${open ? "rotate-180" : ""}`}
        />
      </button>
      {open && (
        <div className="absolute top-full left-0 right-0 mt-px max-h-[calc(100vh-8rem)] overflow-auto border-b bg-background shadow-md animate-in slide-in-from-top-2 fade-in-0 duration-200">
          <div className="p-4">
            <HelpBody entry={props.entry} onNavigate={() => setOpen(false)} />
          </div>
        </div>
      )}
    </div>
  )
}

function HelpBody(props: { entry: HelpEntry; onNavigate?: () => void }) {
  const { entry } = props

  return (
    <div className="flex flex-col gap-4 text-[0.9375rem]">
      <p className="flex items-center gap-2 font-medium">
        <icons.Help className="size-4" />
        <Trans>Help</Trans>
      </p>

      <p className="text-muted-foreground leading-relaxed">{entry.summary}.</p>

      {entry.steps && entry.steps.length > 0 && (
        <ol className="flex flex-col gap-2 list-decimal ps-4 marker:text-muted-foreground">
          {entry.steps.map((step, index) => (
            <li key={index} className="text-muted-foreground leading-relaxed ps-1">
              {step}.
            </li>
          ))}
        </ol>
      )}

      {entry.result && (
        <div>
          <p className="font-medium">
            <Trans>Result</Trans>
          </p>
          <p className="text-muted-foreground leading-relaxed">{entry.result}.</p>
        </div>
      )}

      <a
        href={entry.link.url}
        target="_blank"
        rel="noopener noreferrer"
        onClick={props.onNavigate}
        className="inline-flex items-center gap-1.5 hover:underline"
      >
        <icons.Book className="size-3.5" />
        {entry.link.label}
      </a>
    </div>
  )
}
