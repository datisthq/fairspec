import type { Dialect, LineTerminator, Metadata, Quote } from "./metadata.ts"
import type { PotentialDialect } from "./potentialDialects.ts"
import { detectLineTerminator, generatePotentialDialects } from "./potentialDialects.ts"
import type { SampleSize } from "./sample.ts"
import { findBestDialect, scoreDialect } from "./score.ts"
import { Table } from "./table.ts"

export class Sniffer {
  private sampleSize: SampleSize = { type: "Bytes", count: 8192 }
  private forcedDelimiter?: number
  private forcedQuote?: Quote

  withSampleSize(size: SampleSize) {
    this.sampleSize = size
    return this
  }

  withDelimiter(delimiter: number) {
    this.forcedDelimiter = delimiter
    return this
  }

  withQuote(quote: Quote) {
    this.forcedQuote = quote
    return this
  }

  sniffBytes(bytes: Uint8Array): Metadata {
    const bytesNoBom = this.skipBom(bytes)
    let data = bytesNoBom

    const { data: withoutCommentPreamble, preambleLines: commentPreambleLines } =
      this.skipPreamble(data)
    data = withoutCommentPreamble

    const sample = this.takeSample(data)

    const lineTerminator = detectLineTerminator(sample)

    const dialects = this.forcedDelimiter
      ? this.generateForcedDialects(lineTerminator)
      : generatePotentialDialects(lineTerminator)

    const scores = dialects.map(dialect => scoreDialect(sample, dialect))

    const bestScore = findBestDialect(scores, {
      preferCommonDelimiters: true,
      preferDoubleQuote: true,
    })

    const dialect: Dialect = {
      delimiter: bestScore.dialect.delimiter,
      quote: bestScore.dialect.quote,
      flexible: !bestScore.isUniform,
      isUtf8: true,
      lineTerminator: bestScore.dialect.lineTerminator,
      header: {
        hasHeaderRow: false,
        numPreambleRows: commentPreambleLines,
      },
    }

    const structuralPreambleRows = this.detectStructuralPreamble(data, dialect)
    dialect.header.numPreambleRows += structuralPreambleRows

    const dataAfterAllPreamble = this.skipLines(data, structuralPreambleRows)
    const headerDetectionResult = this.detectHeader(dataAfterAllPreamble, dialect)
    dialect.header.hasHeaderRow = headerDetectionResult.hasHeader

    return this.buildMetadata(bytesNoBom, dialect)
  }

  sniffRows(rows: unknown[][]): Metadata {
    const csvString = this.rowsToCSV(rows)
    const bytes = new TextEncoder().encode(csvString)
    return this.sniffBytes(bytes)
  }

  private valueToString(value: unknown): string {
    if (value === null || value === undefined) {
      return ""
    }
    if (typeof value === "string") {
      return value
    }
    if (typeof value === "number" || typeof value === "boolean") {
      return String(value)
    }
    if (value instanceof Date) {
      return value.toISOString()
    }
    if (typeof value === "object") {
      return JSON.stringify(value)
    }
    return String(value)
  }

  private escapeField(value: string): string {
    const needsQuoting = /[,"\n\r]/.test(value)

    if (!needsQuoting) {
      return value
    }

    const escaped = value.replace(/"/g, '""')
    return `"${escaped}"`
  }

  private rowsToCSV(rows: unknown[][]): string {
    if (rows.length === 0) {
      return ""
    }

    const lines: string[] = []

    for (const row of rows) {
      const values = row.map(value => {
        const stringValue = this.valueToString(value)
        return this.escapeField(stringValue)
      })
      lines.push(values.join(","))
    }

    return lines.join("\n")
  }

  private skipBom(bytes: Uint8Array): Uint8Array {
    if (
      bytes.length >= 3 &&
      bytes[0] === 0xef &&
      bytes[1] === 0xbb &&
      bytes[2] === 0xbf
    ) {
      return bytes.slice(3)
    }
    return bytes
  }

  private skipPreamble(bytes: Uint8Array): {
    data: Uint8Array
    preambleLines: number
  } {
    let lineCount = 0
    let i = 0

    while (i < bytes.length) {
      if (bytes[i] === 35) {
        while (i < bytes.length && bytes[i] !== 10 && bytes[i] !== 13) {
          i++
        }

        if (i < bytes.length && bytes[i] === 13 && bytes[i + 1] === 10) {
          i += 2
        } else if (i < bytes.length) {
          i++
        }

        lineCount++
      } else {
        break
      }
    }

    return {
      data: bytes.slice(i),
      preambleLines: lineCount,
    }
  }

  private takeSample(bytes: Uint8Array): Uint8Array {
    if (this.sampleSize.type === "All") {
      return bytes
    }

    if (this.sampleSize.type === "Bytes") {
      return bytes.slice(0, this.sampleSize.count)
    }

    let lineCount = 0
    let i = 0

    while (i < bytes.length && lineCount < this.sampleSize.count) {
      if (bytes[i] === 10) {
        lineCount++
      } else if (bytes[i] === 13) {
        if (i + 1 < bytes.length && bytes[i + 1] === 10) {
          i++
        }
        lineCount++
      }
      i++
    }

    return bytes.slice(0, i)
  }

  private generateForcedDialects(lineTerminator: LineTerminator): PotentialDialect[] {
    if (!this.forcedDelimiter) {
      throw new Error("generateForcedDialects called without forcedDelimiter")
    }

    const delimiter = this.forcedDelimiter

    const quotes: Quote[] = this.forcedQuote
      ? [this.forcedQuote]
      : [{ type: "None" }, { type: "Some", char: 34 }, { type: "Some", char: 39 }]

    return quotes.map(quote => ({
      delimiter,
      quote,
      lineTerminator,
    }))
  }

  private detectStructuralPreamble(bytes: Uint8Array, dialect: Dialect): number {
    const table = Table.parse(bytes, {
      delimiter: dialect.delimiter,
      quote: dialect.quote,
      lineTerminator: dialect.lineTerminator,
    })

    if (table.numRows() < 2) {
      return 0
    }

    const modalFieldCount = table.getModalFieldCount()
    let preambleRows = 0

    for (let i = 0; i < table.fieldCounts.length; i++) {
      const count = table.fieldCounts[i]
      if (count === modalFieldCount) {
        break
      }
      preambleRows++
    }

    if (preambleRows >= table.numRows()) {
      return 0
    }

    return preambleRows
  }

  private skipLines(bytes: Uint8Array, numLines: number): Uint8Array {
    if (numLines === 0) {
      return bytes
    }

    let lineCount = 0
    let i = 0

    while (i < bytes.length && lineCount < numLines) {
      if (bytes[i] === 10) {
        lineCount++
        i++
      } else if (bytes[i] === 13) {
        if (i + 1 < bytes.length && bytes[i + 1] === 10) {
          i += 2
        } else {
          i++
        }
        lineCount++
      } else {
        i++
      }
    }

    return bytes.slice(i)
  }

  private detectHeader(bytes: Uint8Array, dialect: Dialect): { hasHeader: boolean } {
    const table = Table.parse(bytes, {
      delimiter: dialect.delimiter,
      quote: dialect.quote,
      lineTerminator: dialect.lineTerminator,
    })

    if (table.numRows() < 2) {
      return { hasHeader: false }
    }

    const firstRow = table.rows[0]
    if (!firstRow) {
      return { hasHeader: false }
    }

    let headerScore = 0

    for (const field of firstRow) {
      const trimmed = field.trim()

      if (trimmed.length === 0) {
        headerScore -= 0.2
        continue
      }

      if (/^(true|false)$/i.test(trimmed)) {
        headerScore -= 0.3
        continue
      }

      if (/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(trimmed)) {
        headerScore += 0.3
      }

      if (/[A-Z]/.test(trimmed) && !/^\d+$/.test(trimmed)) {
        headerScore += 0.2
      }

      if (/[_\s-]/.test(trimmed)) {
        headerScore += 0.1
      }

      if (/^\d+(\.\d+)?$/.test(trimmed)) {
        headerScore -= 0.3
      }
    }

    return { hasHeader: headerScore > 0 }
  }

  private buildMetadata(bytes: Uint8Array, dialect: Dialect): Metadata {
    const dataAfterPreamble = this.skipLines(bytes, dialect.header.numPreambleRows)

    const table = Table.parse(dataAfterPreamble, {
      delimiter: dialect.delimiter,
      quote: dialect.quote,
      lineTerminator: dialect.lineTerminator,
    })

    const dataFieldCounts =
      dialect.header.hasHeaderRow && table.fieldCounts.length > 0
        ? table.fieldCounts.slice(1)
        : table.fieldCounts

    const isUniform =
      dataFieldCounts.length === 0 ||
      dataFieldCounts.every(count => count === dataFieldCounts[0])

    dialect.flexible = !isUniform

    const numFields = table.getModalFieldCount()

    let fields: string[] = []
    if (dialect.header.hasHeaderRow && table.numRows() > 0) {
      const headerRow = table.rows[0]
      fields = headerRow ? headerRow.map(field => field.trim()) : []
    } else {
      fields = Array.from({ length: numFields }, (_, i) => `field_${i + 1}`)
    }

    const totalBytes = dataAfterPreamble.length
    const totalRows = table.numRows()
    const avgRecordLen = totalRows > 0 ? totalBytes / totalRows : 0

    return {
      dialect,
      avgRecordLen,
      numFields,
      fields,
    }
  }
}
