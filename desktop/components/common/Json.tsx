import JsonView from "@uiw/react-json-view"
import { githubDarkTheme } from "@uiw/react-json-view/githubDark"
import { githubLightTheme } from "@uiw/react-json-view/githubLight"
import { useEffect, useState } from "react"

export function Json({ value }: { value: unknown }) {
  const [isDark, setIsDark] = useState(false)

  useEffect(() => {
    const checkDarkMode = () => {
      setIsDark(document.documentElement.classList.contains("dark"))
    }

    checkDarkMode()

    const observer = new MutationObserver(checkDarkMode)
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["class"],
    })

    return () => observer.disconnect()
  }, [])

  return (
    <div className="border p-4 rounded-lg overflow-auto">
      {/* @ts-expect-error */}
      <JsonView
        value={value}
        displayDataTypes={false}
        style={isDark ? githubDarkTheme : githubLightTheme}
      />
    </div>
  )
}
