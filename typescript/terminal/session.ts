import { setTimeout } from "node:timers/promises"
import util from "node:util"
import type { Frame, Report } from "@fairspec/library"
import { FairspecException } from "@fairspec/library"
import { colorize } from "json-colorizer"
import pc from "picocolors"
import type { TaskInnerAPI } from "tasuku"
import tasuku from "tasuku"
import { renderError } from "./actions/error/render.ts"

type TaskApi = Omit<TaskInnerAPI, "task">
type Status = "success" | "warning" | "error"

export interface SessionOptions {
  silent?: boolean
  debug?: boolean
  json?: boolean
}

export class Session implements SessionOptions {
  silent?: boolean
  debug?: boolean
  json?: boolean

  constructor(options: SessionOptions) {
    this.silent = options.silent
    this.debug = options.debug
    this.json = options.json

    // Have empty line before/after output
    if (!this.silent && !this.json) {
      process.stdout.write("\n")
      process.once("exit", () => {
        process.stdout.write("\n")
      })
    }
  }

  renderText(text: string, options?: { status?: Status }) {
    if (this.silent || this.json) {
      return
    }

    if (!options?.status) {
      process.stdout.write(text)
      process.stdout.write("\n")
      return
    }

    process.stdout.write(`${renderStatus(options.status)} ${text}`)
    process.stdout.write("\n")
  }

  renderTextResult(text: string, options?: { status?: Status }) {
    if (this.silent) {
      return
    }

    if (this.json) {
      text = util.stripVTControlCharacters(text)
      process.stdout.write(JSON.stringify({ result: text }, null, 2))
      process.stdout.write("\n")
      return
    }

    if (!options?.status) {
      process.stdout.write(text)
      process.stdout.write("\n")
      return
    }

    process.stdout.write(`${renderStatus(options.status)} ${text}`)
    process.stdout.write("\n")
  }

  renderDataResult(data: string | object) {
    if (this.silent) {
      return
    }

    if (this.json) {
      process.stdout.write(JSON.stringify(data, null, 2))
      process.stdout.write("\n")
      return
    }

    process.stdout.write(colorize(data))
    process.stdout.write("\n")
  }

  renderFrameResult(frame: Frame) {
    if (this.silent) {
      return
    }

    if (this.json) {
      process.stdout.write(JSON.stringify(frame.toRecords(), null, 2))
      process.stdout.write("\n")
      return
    }

    process.stdout.write(frame.toString())
    process.stdout.write("\n")
  }

  renderReportResult(report: Report) {
    if (!report.valid) {
      process.exitCode = 1
    }

    if (this.silent) {
      return
    }

    if (this.json) {
      process.stdout.write(JSON.stringify(report, null, 2))
      process.stdout.write("\n")
      return
    }

    if (report.valid) {
      this.renderText("Validation passed", { status: "success" })
      return
    }

    for (const error of report.errors) {
      this.renderText(renderError(error), { status: "error" })
    }
  }

  async task<T>(title: string, func: (api: TaskApi) => Promise<T>) {
    const runTask = async (api: TaskApi): Promise<T> => {
      try {
        return await func(api)
      } catch (exception) {
        if (this.debug) {
          throw exception
        }

        // Wihout timeout, tasulu clears the failed task result
        api.setError(exception instanceof Error ? exception : String(exception))
        await setTimeout(100)

        if (exception instanceof FairspecException) {
          if (exception.report) {
            process.stdout.write("\n")
            this.renderReportResult(exception.report)
          }
        }

        process.exit(1)
      }
    }

    if (this.json || this.silent) {
      return await runTask({
        setTitle: () => {},
        setStatus: () => {},
        setOutput: () => {},
        setWarning: () => {},
        setError: error => {
          if (this.json) {
            process.stdout.write(JSON.stringify({ error: String(error) }, null, 2))
          }

          process.exit(1)
        },
      })
    }

    const response = await tasuku(title, runTask)
    if (response.state === "success") response.clear()
    return response.result
  }
}

// For some reason, biome replaces inline emojis
function renderStatus(status: Status) {
  switch (status) {
    case "success":
      return pc.green("\u2714")
    case "warning":
      return pc.yellow("\u26A0")
    case "error":
      return pc.red("\u2716")
  }
}
