import { useLingui } from "@lingui/react/macro"
import { useHotkey } from "@tanstack/react-hotkeys"
import { Button } from "#elements/button.tsx"
import { getCurrentTheme, setTheme } from "#helpers/theme.ts"
import * as icons from "#icons.ts"

export function Theme() {
  const { t } = useLingui()

  const handleToggle = async () => {
    const oldTheme = getCurrentTheme()
    const newTheme = oldTheme === "light" ? "dark" : "light"
    await setTheme(newTheme)
  }

  useHotkey("T", handleToggle)

  return (
    <Button
      variant="ghost"
      size="sm"
      onClick={handleToggle}
      title={t`Change Theme`}
      className="w-full h-auto rounded-xl cursor-pointer text-xs font-normal justify-start text-muted-foreground bg-sidebar-accent/70 border border-border shadow-xs hover:!bg-background px-3 py-2"
    >
      <div className="flex flex-1 gap-2 items-center dark:hidden">
        <icons.LightTheme className="size-4" />
        <span className="flex-1 text-left">{t`Light Theme`}</span>
        <kbd className="rounded border border-border bg-muted px-1.5 py-0.5 text-[10px] font-mono">
          T
        </kbd>
      </div>
      <div className="flex-1 gap-2 items-center hidden dark:flex">
        <icons.DarkTheme className="size-4" />
        <span className="flex-1 text-left">{t`Dark Theme`}</span>
        <kbd className="rounded border border-border bg-muted px-1.5 py-0.5 text-[10px] font-mono">
          T
        </kbd>
      </div>
    </Button>
  )
}
