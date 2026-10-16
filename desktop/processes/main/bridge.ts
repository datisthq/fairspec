import { writeFile } from "node:fs/promises"
import { basename, dirname, join } from "node:path"
import { engineRpcHandler } from "#processes/main/engine.ts"
import type { LanguageId } from "#constants/language.ts"
import { BrowserWindow, dialog, ipcMain } from "electron"
import { store } from "#processes/main/store.ts"
import * as settings from "#settings.ts"

// Paths the user has picked in a save dialog. The renderer may only write to
// these, so a write cannot be aimed at an arbitrary location on disk.
const savedFilePaths = new Set<string>()

export function createBridge() {
  ipcMain.on(settings.ENGINE_IPC, async event => {
    const [serverPort] = event.ports

    if (serverPort) {
      engineRpcHandler.upgrade(serverPort)
      serverPort.start()
    }
  })

  ipcMain.handle(
    "dialog:openFile",
    async (_, options: { filters?: { name: string; extensions: string[] }[] }) => {
      const lastOpenedFolder = store.get("lastOpenedFolder") as string | undefined

      const result = await dialog.showOpenDialog({
        properties: ["openFile"],
        filters: options.filters,
        defaultPath: lastOpenedFolder,
      })

      if (result.canceled || result.filePaths.length === 0) {
        return undefined
      }

      const selectedFile = result.filePaths[0]
      if (selectedFile) {
        const folderPath = dirname(selectedFile)
        store.set("lastOpenedFolder", folderPath)
      }

      return selectedFile
    },
  )

  ipcMain.handle(
    "dialog:saveFile",
    async (
      _,
      options: {
        defaultPath?: string
        filters?: { name: string; extensions: string[] }[]
      },
    ) => {
      const lastOpenedFolder = store.get("lastOpenedFolder") as string | undefined

      const result = await dialog.showSaveDialog({
        defaultPath: options.defaultPath
          ? join(lastOpenedFolder || "", basename(options.defaultPath))
          : lastOpenedFolder,
        filters: options.filters,
      })

      if (result.canceled || !result.filePath) {
        return undefined
      }

      const selectedFile = result.filePath
      const folderPath = dirname(selectedFile)
      store.set("lastOpenedFolder", folderPath)
      savedFilePaths.add(selectedFile)

      return selectedFile
    },
  )

  ipcMain.handle(
    "file:write",
    async (_, options: { filePath: string; content: string }) => {
      if (!savedFilePaths.has(options.filePath)) {
        throw new Error("Refusing to write to a path the user did not choose")
      }

      await writeFile(options.filePath, options.content, "utf-8")
      return options.filePath
    },
  )

  ipcMain.handle("theme:get", async () => {
    const theme = store.get("theme") as "light" | "dark" | undefined
    return theme || "light"
  })

  ipcMain.handle("theme:set", async (_, theme: "light" | "dark") => {
    store.set("theme", theme)
    return theme
  })

  ipcMain.handle("language:get", async () => {
    const languageId = store.get("languageId") as LanguageId | null
    return languageId ?? undefined
  })

  ipcMain.handle("language:set", async (_, languageId: LanguageId) => {
    store.set("languageId", languageId ?? null)
    return languageId
  })

  ipcMain.handle("window:toggleMaximize", event => {
    const win = BrowserWindow.fromWebContents(event.sender)
    if (!win) return
    win.setFullScreen(!win.isFullScreen())
  })

  ipcMain.handle("window:isFullScreen", event => {
    return BrowserWindow.fromWebContents(event.sender)?.isFullScreen() ?? false
  })
}
