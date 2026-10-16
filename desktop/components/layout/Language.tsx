import { useLingui } from "@lingui/react/macro"
import { De, Es, Fr, Gb, It, Pt, Ru, Ua } from "react-flags-select"
import { Languages, type Language as LanguageType } from "#constants/language.ts"
import { Button } from "#elements/button.tsx"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "#elements/dropdown-menu.tsx"
import { activateLocale, setLanguage } from "#helpers/locale.ts"
import * as icons from "#icons.ts"
import * as settings from "#settings.ts"

const LANGUAGE_FLAGS = {
  de: De,
  en: Gb,
  es: Es,
  fr: Fr,
  it: It,
  pt: Pt,
  ru: Ru,
  uk: Ua,
} as const

export function Language() {
  const { t } = useLingui()

  const onLanguageChange = async (language: LanguageType) => {
    await activateLocale(language.id)
    await setLanguage(language.id)
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        render={
          <Button
            variant="ghost"
            size="sm"
            title={t`Change Language`}
            className="w-full h-auto rounded-xl cursor-pointer text-xs font-normal justify-start text-muted-foreground bg-sidebar-accent/70 border border-border shadow-xs hover:!bg-background px-3 py-2"
          />
        }
      >
        <div className="flex flex-1 gap-2 items-center">
          <icons.Language className="size-4" />
          <span className="flex-1 text-left">{t`Language`}</span>
        </div>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        align="start"
        className="flex flex-col gap-1 p-2 w-(--radix-dropdown-menu-trigger-width)"
      >
        {Object.values(Languages).map(language => {
          const ItemFlag = LANGUAGE_FLAGS[language.id]
          return (
            <DropdownMenuItem
              key={language.id}
              onClick={() => onLanguageChange(language)}
              className="cursor-pointer"
            >
              <div className="flex gap-2 flex-nowrap items-center cursor-pointer">
                <ItemFlag fontSize={settings.ICON_SIZE} />
                {language.title}
              </div>
            </DropdownMenuItem>
          )
        })}
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
