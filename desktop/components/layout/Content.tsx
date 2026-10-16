import { useHelp } from "#helpers/help.ts"
import { Help, MobileHelp } from "./Help.tsx"

export function Content(props: { children: React.ReactNode }) {
  const entry = useHelp()

  return (
    <>
      <MobileHelp entry={entry} />
      <div className="flex flex-1 gap-10 p-4 pt-8 md:p-10">
        <div className="flex-1 min-w-0 mx-auto max-w-3xl">{props.children}</div>
        <Help entry={entry} />
      </div>
    </>
  )
}
