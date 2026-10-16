import { useLingui } from "@lingui/react/macro"
import logo from "#assets/logo.svg"

export function Logo() {
  const { t } = useLingui()
  const title = t`Fairspec Application`
  const description = t`Data management application`

  return (
    <div className="flex items-end gap-2.5 text-sm">
      <img src={logo} alt={title} className="size-6.5 mb-1.25" />
      <div className="flex flex-col gap-0.5 leading-none">
        <span className="font-semibold">{title}</span>
        <span className="text-xs opacity-80">{description}</span>
      </div>
    </div>
  )
}
