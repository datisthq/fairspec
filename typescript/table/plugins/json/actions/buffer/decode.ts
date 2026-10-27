import type { Buffer } from "node:buffer"

export function decodeJsonBuffer(buffer: Buffer, options: { isLines: boolean }) {
  const text = buffer.toString("utf-8")
  const string = text.startsWith("\uFEFF") ? text.slice(1) : text

  return options.isLines
    ? string
        .split("\n")
        .map(line => (line.endsWith("\r") ? line.slice(0, -1) : line))
        .filter(Boolean)
        .map(line => JSON.parse(line))
    : JSON.parse(string)
}
