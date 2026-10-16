import type { Quote } from "./metadata.ts"
import type { PotentialDialect } from "./potentialDialects.ts"
import { Table } from "./table.ts"
import { calculateTau0, calculateTau1 } from "./uniformity.ts"

export interface DialectScore {
  dialect: PotentialDialect
  gamma: number
  tau0: number
  tau1: number
  numFields: number
  isUniform: boolean
}

interface QuoteEvidence {
  quoteDensity: number
  boundaryMatches: number
  internalMatches: number
}

export function scoreDialect(bytes: Uint8Array, dialect: PotentialDialect): DialectScore {
  const table = Table.parse(bytes, dialect)

  const tau0 = calculateTau0(table.fieldCounts)
  const tau1 = calculateTau1(table.fieldCounts, table.getModalFieldCount())
  const numFields = table.getModalFieldCount()
  const isUniform = table.isUniform()
  const numRows = table.numRows()

  const quoteEvidence = analyzeQuoteEvidence(bytes, dialect)

  const gamma = calculateGamma(
    tau0,
    tau1,
    numRows,
    numFields,
    isUniform,
    dialect.delimiter,
    dialect.quote,
    quoteEvidence,
  )

  return {
    dialect,
    gamma,
    tau0,
    tau1,
    numFields,
    isUniform,
  }
}

function calculateGamma(
  tau0: number,
  tau1: number,
  numRows: number,
  numFields: number,
  isUniform: boolean,
  delimiter: number,
  quote: Quote,
  quoteEvidence: QuoteEvidence,
): number {
  let gamma = tau0 * 0.4 + tau1 * 0.6

  if (isUniform) {
    gamma += 0.2
  }

  if (numFields >= 2 && numFields <= 50) {
    gamma += 0.3
  } else if (numFields === 1) {
    gamma -= 1.0
  }

  if (numRows >= 2) {
    gamma += 0.05
  }

  const commonDelimiters = [44, 9, 59, 124]
  if (commonDelimiters.includes(delimiter)) {
    gamma += 0.15
  }

  if (quote.type === "Some") {
    const quoteScore =
      quoteEvidence.boundaryMatches * 0.5 +
      quoteEvidence.quoteDensity * 0.3 -
      quoteEvidence.internalMatches * 0.2

    gamma += Math.max(0, Math.min(0.2, quoteScore))

    if (quote.char === 34) {
      gamma += 0.05
    }
  } else {
    if (quoteEvidence.quoteDensity < 0.01) {
      gamma += 0.1
    }
  }

  return Math.max(0, Math.min(2, gamma))
}

function analyzeQuoteEvidence(
  bytes: Uint8Array,
  dialect: PotentialDialect,
): QuoteEvidence {
  if (dialect.quote.type === "None") {
    return {
      quoteDensity: 0,
      boundaryMatches: 0,
      internalMatches: 0,
    }
  }

  const quoteChar = dialect.quote.char
  const delimiterChar = dialect.delimiter

  let quoteCount = 0
  let boundaryMatches = 0
  let internalMatches = 0

  for (let i = 0; i < bytes.length; i++) {
    if (bytes[i] === quoteChar) {
      quoteCount++

      const prevChar = i > 0 ? bytes[i - 1] : undefined
      const nextChar = i < bytes.length - 1 ? bytes[i + 1] : undefined

      const atBoundary =
        prevChar === undefined ||
        prevChar === delimiterChar ||
        prevChar === 10 ||
        prevChar === 13 ||
        nextChar === undefined ||
        nextChar === delimiterChar ||
        nextChar === 10 ||
        nextChar === 13

      if (atBoundary) {
        boundaryMatches++
      } else {
        internalMatches++
      }
    }
  }

  const quoteDensity = bytes.length > 0 ? quoteCount / bytes.length : 0

  return {
    quoteDensity,
    boundaryMatches,
    internalMatches,
  }
}

export function findBestDialect(
  scores: DialectScore[],
  preferences: {
    preferCommonDelimiters: boolean
    preferDoubleQuote: boolean
  },
): DialectScore {
  if (scores.length === 0) {
    throw new Error("No dialect scores provided")
  }

  const firstScore = scores[0]
  if (!firstScore) {
    throw new Error("No dialect scores provided")
  }

  let bestScore = firstScore

  for (const score of scores) {
    let currentGamma = score.gamma
    let bestGamma = bestScore.gamma

    if (preferences.preferCommonDelimiters) {
      const commonDelimiters = [44, 9, 59, 124]
      if (commonDelimiters.includes(score.dialect.delimiter)) {
        currentGamma += 0.05
      }
      if (commonDelimiters.includes(bestScore.dialect.delimiter)) {
        bestGamma += 0.05
      }
    }

    if (preferences.preferDoubleQuote) {
      if (score.dialect.quote.type === "Some" && score.dialect.quote.char === 34) {
        currentGamma += 0.05
      }
      if (
        bestScore.dialect.quote.type === "Some" &&
        bestScore.dialect.quote.char === 34
      ) {
        bestGamma += 0.05
      }
    }

    if (currentGamma > bestGamma) {
      bestScore = score
    }
  }

  return bestScore
}
