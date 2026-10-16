import { useLingui } from "@lingui/react/macro"
import { useEffect, useState } from "react"
import { Button } from "#elements/button.tsx"
import * as icons from "#icons.ts"

export function Maximize() {
  const { t } = useLingui()
  const [isFullScreen, setIsFullScreen] = useState(false)

  useEffect(() => {
    globalThis.desktop.isFullScreen().then(setIsFullScreen)
    return globalThis.desktop.onFullScreenChange(setIsFullScreen)
  }, [])

  const handleToggle = () => {
    globalThis.desktop.toggleMaximize()
  }

  const Icon = isFullScreen ? icons.Restore : icons.Maximize
  const label = isFullScreen ? t`Exit Fullscreen` : t`Enter Fullscreen`

  return (
    <Button
      variant="ghost"
      size="icon-sm"
      title={label}
      aria-label={label}
      onClick={handleToggle}
    >
      <Icon />
    </Button>
  )
}
