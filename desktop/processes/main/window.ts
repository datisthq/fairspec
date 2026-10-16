import { join } from "node:path"
import { BrowserWindow, shell } from "electron"
// electron-vite's ?asset query has no ambient type declaration
// @ts-expect-error
import iconPath from "#assets/logo.svg?asset"
import * as settings from "#settings.ts"
import { store } from "./store.ts"

const rendererUrl = process.env.ELECTRON_RENDERER_URL

function isInternalUrl(url: string) {
  if (rendererUrl && url.startsWith(rendererUrl)) return true
  return url.startsWith("file://")
}

export function createWindow() {
  const preloadFolder = join(import.meta.dirname, "..", "preload")

  const mainWindow = new BrowserWindow({
    show: false,
    frame: false,
    ...(process.platform === "linux" ? { icon: iconPath } : {}),
    webPreferences: {
      preload: join(preloadFolder, "preload.js"),
      contextIsolation: true,
    },
  })

  if (rendererUrl) {
    mainWindow.loadURL(rendererUrl)
  } else {
    mainWindow.loadFile(join(import.meta.dirname, "..", "renderer", "index.html"))
  }

  mainWindow.webContents.setWindowOpenHandler(({ url }) => {
    if (!isInternalUrl(url)) {
      shell.openExternal(url)
      return { action: "deny" }
    }
    return { action: "allow" }
  })

  mainWindow.webContents.on("will-navigate", (event, url) => {
    if (!isInternalUrl(url)) {
      event.preventDefault()
      shell.openExternal(url)
    }
  })

  mainWindow.on("enter-full-screen", () => {
    mainWindow.webContents.send("window:fullScreenChanged", true)
  })
  mainWindow.on("leave-full-screen", () => {
    mainWindow.webContents.send("window:fullScreenChanged", false)
  })

  const zoomFactor = store.get("zoomFactor") ?? 1.0
  const zoomLevels = [0.5, 0.67, 0.75, 0.8, 0.9, 1.0, 1.1, 1.25, 1.5, 1.75, 2.0]

  mainWindow.once("ready-to-show", () => {
    mainWindow.webContents.setZoomFactor(zoomFactor)
    mainWindow.setTitle(settings.APP_NAME)
    mainWindow.setMenu(null)
    mainWindow.maximize()
    mainWindow.show()
  })

  mainWindow.webContents.on("before-input-event", (event, input) => {
    if (input.control) {
      if (input.key === "+" || input.key === "=") {
        const currentZoom = mainWindow.webContents.getZoomFactor()
        const nextLevel = zoomLevels.find(level => level > currentZoom)
        if (nextLevel) {
          mainWindow.webContents.setZoomFactor(nextLevel)
          store.set("zoomFactor", nextLevel)
        }
        event.preventDefault()
      } else if (input.key === "-" || input.key === "_") {
        const currentZoom = mainWindow.webContents.getZoomFactor()
        const prevLevel = [...zoomLevels].reverse().find(level => level < currentZoom)
        if (prevLevel) {
          mainWindow.webContents.setZoomFactor(prevLevel)
          store.set("zoomFactor", prevLevel)
        }
        event.preventDefault()
      } else if (input.key === "0") {
        mainWindow.webContents.setZoomFactor(1.0)
        store.set("zoomFactor", 1.0)
        event.preventDefault()
      }
    }
  })

  return mainWindow
}
