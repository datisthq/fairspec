import { useLingui } from "@lingui/react/macro"
import { Button } from "#elements/button.tsx"
import * as icons from "#icons.ts"

export function Close() {
  const { t } = useLingui()

  const handleClose = () => {
    globalThis.window?.close()
  }

  return (
    <Button
      variant="ghost"
      size="icon-sm"
      title={t`Close Application`}
      aria-label={t`Close Application`}
      onClick={handleClose}
    >
      <icons.Close />
    </Button>
  )
}
