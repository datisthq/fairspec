import { contextBridge, ipcRenderer } from "electron"
import * as settings from "#settings.ts"
import type { LanguageId } from "#constants/language.ts"

contextBridge.exposeInMainWorld("desktop", {
  engineIpc: settings.ENGINE_IPC,

  openFileDialog: async (options: {
    filters?: {
      name: string
      extensions: string[]
    }[]
  }) => {
    return await ipcRenderer.invoke("dialog:openFile", options)
  },

  saveFileDialog: async (options: {
    defaultPath?: string
    filters?: { name: string; extensions: string[] }[]
  }) => {
    return await ipcRenderer.invoke("dialog:saveFile", options)
  },

  writeFile: async (options: { filePath: string; content: string }) => {
    return await ipcRenderer.invoke("file:write", options)
  },

  getTheme: async () => {
    return await ipcRenderer.invoke("theme:get")
  },

  setTheme: async (theme: "light" | "dark") => {
    return await ipcRenderer.invoke("theme:set", theme)
  },

  getLanguage: async (): Promise<LanguageId> => {
    return await ipcRenderer.invoke("language:get")
  },

  setLanguage: async (languageId: LanguageId): Promise<LanguageId> => {
    return await ipcRenderer.invoke("language:set", languageId)
  },

  toggleMaximize: async () => {
    await ipcRenderer.invoke("window:toggleMaximize")
  },

  isFullScreen: async (): Promise<boolean> => {
    return await ipcRenderer.invoke("window:isFullScreen")
  },

  onFullScreenChange: (callback: (isFullScreen: boolean) => void) => {
    const listener = (_: unknown, isFullScreen: boolean) => callback(isFullScreen)
    ipcRenderer.on("window:fullScreenChanged", listener)
    return () => {
      ipcRenderer.removeListener("window:fullScreenChanged", listener)
    }
  },
})

window.addEventListener("message", event => {
  if (event.data === settings.ENGINE_IPC) {
    const [serverPort] = event.ports

    if (serverPort) {
      ipcRenderer.postMessage(settings.ENGINE_IPC, null, [serverPort])
    }
  }
})
