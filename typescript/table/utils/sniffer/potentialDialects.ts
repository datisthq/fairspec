import type { LineTerminator, Quote } from "./metadata.ts"

export interface PotentialDialect {
  delimiter: number
  quote: Quote
  lineTerminator: LineTerminator
}

export const DELIMITERS: number[] = [
  44, // , (comma)
  59, // ; (semicolon)
  9, // \t (tab)
  124, // | (pipe)
  94, // ^ (caret)
  126, // ~ (tilde)
  35, // # (hash)
  38, // & (ampersand)
  167, // § (section)
  47, // / (slash)
]

export const QUOTE_CHARS: Quote[] = [
  { type: "None" },
  { type: "Some", char: 34 }, // " (double quote)
  { type: "Some", char: 39 }, // ' (single quote)
]

export function detectLineTerminator(bytes: Uint8Array): LineTerminator {
  let hasCR = false
  let hasLF = false
  let hasCRLF = false

  for (let i = 0; i < bytes.length; i++) {
    if (bytes[i] === 13) {
      if (i + 1 < bytes.length && bytes[i + 1] === 10) {
        hasCRLF = true
        i++
      } else {
        hasCR = true
      }
    } else if (bytes[i] === 10) {
      hasLF = true
    }
  }

  if (hasCRLF) return "CRLF"
  if (hasLF) return "LF"
  if (hasCR) return "CR"

  return "LF"
}

export function generatePotentialDialects(
  lineTerminator: LineTerminator,
): PotentialDialect[] {
  const dialects: PotentialDialect[] = []

  for (const delimiter of DELIMITERS) {
    for (const quote of QUOTE_CHARS) {
      dialects.push({
        delimiter,
        quote,
        lineTerminator,
      })
    }
  }

  return dialects
}
