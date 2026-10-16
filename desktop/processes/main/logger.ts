import { ElectronLogTransport } from "@loglayer/transport-electron-log"
import log from "electron-log/main.js"
import { LogLayer } from "loglayer"

export const logger = new LogLayer({
  transport: new ElectronLogTransport({
    logger: log,
  }),
})
