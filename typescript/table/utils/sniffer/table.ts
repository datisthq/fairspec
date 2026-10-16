import type { PotentialDialect } from "./potentialDialects.ts"

export class Table {
  rows: string[][]
  fieldCounts: number[]
  private cachedModalFieldCount?: number

  constructor(rows: string[][], fieldCounts: number[]) {
    this.rows = rows
    this.fieldCounts = fieldCounts
  }

  static parse(bytes: Uint8Array, dialect: PotentialDialect): Table {
    const decoder = new TextDecoder("utf-8", { fatal: false })
    const text = decoder.decode(bytes)

    const rows: string[][] = []
    const fieldCounts: number[] = []

    const delimiterChar = String.fromCharCode(dialect.delimiter)
    const quoteChar =
      dialect.quote.type === "Some" ? String.fromCharCode(dialect.quote.char) : null

    const lineTerminatorRegex =
      dialect.lineTerminator === "CRLF"
        ? /\r\n/g
        : dialect.lineTerminator === "CR"
          ? /\r/g
          : /\n/g

    const lines = text.split(lineTerminatorRegex)

    for (const line of lines) {
      if (line.length === 0) continue

      const fields = quoteChar
        ? parseQuotedLine(line, delimiterChar, quoteChar)
        : line.split(delimiterChar)

      rows.push(fields)
      fieldCounts.push(fields.length)
    }

    return new Table(rows, fieldCounts)
  }

  getModalFieldCount(): number {
    if (this.cachedModalFieldCount !== undefined) {
      return this.cachedModalFieldCount
    }

    if (this.fieldCounts.length === 0) {
      this.cachedModalFieldCount = 0
      return 0
    }

    const maxFieldCount = Math.max(...this.fieldCounts)

    if (maxFieldCount <= 256) {
      const frequency = new Array(maxFieldCount + 1).fill(0)
      for (const count of this.fieldCounts) {
        frequency[count]++
      }

      let maxFreq = 0
      let modal = 0
      for (let i = 0; i <= maxFieldCount; i++) {
        if (frequency[i] > maxFreq) {
          maxFreq = frequency[i]
          modal = i
        }
      }

      this.cachedModalFieldCount = modal
      return modal
    } else {
      const frequency = new Map<number, number>()
      for (const count of this.fieldCounts) {
        frequency.set(count, (frequency.get(count) || 0) + 1)
      }

      let maxFreq = 0
      let modal = 0
      for (const [count, freq] of frequency) {
        if (freq > maxFreq) {
          maxFreq = freq
          modal = count
        }
      }

      this.cachedModalFieldCount = modal
      return modal
    }
  }

  isUniform(): boolean {
    if (this.fieldCounts.length === 0) return true

    const first = this.fieldCounts[0]
    return this.fieldCounts.every(count => count === first)
  }

  numRows(): number {
    return this.rows.length
  }
}

function parseQuotedLine(line: string, delimiter: string, quote: string): string[] {
  const fields: string[] = []
  let currentField = ""
  let inQuotes = false
  let i = 0

  while (i < line.length) {
    const char = line[i]

    if (char === quote) {
      if (inQuotes && i + 1 < line.length && line[i + 1] === quote) {
        currentField += quote
        i += 2
      } else {
        inQuotes = !inQuotes
        i++
      }
    } else if (char === delimiter && !inQuotes) {
      fields.push(currentField)
      currentField = ""
      i++
    } else {
      currentField += char
      i++
    }
  }

  fields.push(currentField)
  return fields
}
