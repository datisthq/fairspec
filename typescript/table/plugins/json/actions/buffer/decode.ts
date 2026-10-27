import type { Buffer } from "node:buffer"

export function decodeJsonBuffer(buffer: Buffer, options: { isLines: boolean }) {
  const text = buffer.toString("utf-8")
  const string = text.startsWith("\uFEFF") ? text.slice(1) : text

  return options.isLines
    ? string
        .split("\n")
        .filter(line => !/^[ \t\r]*$/.test(line))
        .map(line => JSON.parse(line))
    : JSON.parse(string)
}
