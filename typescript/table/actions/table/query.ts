import * as pl from "nodejs-polars"
import type { Table } from "../../models/table.ts"

/** Run a SQL query against a table, exposed as `self`. */
export function queryTable(table: Table, query: string) {
  return queryTables({ self: table }, query)
}

/** Run a SQL query against several tables, each exposed under its key. */
export function queryTables(tables: Record<string, Table>, query: string) {
  const context = pl.SQLContext(tables)
  const result = context.execute(setNullsLast(query))

  // The binding returns a SQL error as the frame instead of throwing it
  if (result._ldf instanceof Error) {
    throw result._ldf
  }

  return result
}

// A keyword that ends an ORDER BY list at the depth it opened
const ORDER_END_KEYWORDS = new Set([
  "LIMIT",
  "OFFSET",
  "FETCH",
  "UNION",
  "EXCEPT",
  "INTERSECT",
  "ROWS",
  "RANGE",
  "GROUPS",
])

/**
 * Add `NULLS LAST` to every ORDER BY item that names no null order.
 *
 * Polars sorts nulls first in descending order, while the spec places them last
 * in both directions.
 */
export function setNullsLast(query: string) {
  const positions: number[] = []
  const frames = [createOrderFrame()]
  const size = query.length
  let index = 0

  while (index < size) {
    const char = query[index] ?? ""
    const frame = frames[frames.length - 1] ?? createOrderFrame()

    if (char === "'" || char === '"' || char === "`") {
      index = skipQuoted(query, index)
      markOrderFrame(frame, index, undefined)
    } else if (query.startsWith("--", index)) {
      const end = query.indexOf("\n", index)
      index = end === -1 ? size : end
    } else if (query.startsWith("/*", index)) {
      const end = query.indexOf("*/", index + 2)
      index = end === -1 ? size : end + 2
    } else if (/\s/u.test(char)) {
      index += 1
    } else if (char === "(") {
      frames.push(createOrderFrame())
      index += 1
    } else if (char === ")") {
      const closed = frames.length > 1 ? frames.pop() : undefined
      const parent = frames[frames.length - 1]
      if (closed && parent) {
        finishOrderItem(closed, positions)
        markOrderFrame(parent, index + 1, undefined)
      }
      index += 1
    } else if (char === "," || char === ";") {
      finishOrderItem(frame, positions)
      if (char === ";") frame.isOrdering = false
      frame.lastWord = undefined
      index += 1
    } else if (isWordChar(char)) {
      let end = index
      while (end < size && (isWordChar(query[end] ?? "") || query[end] === "$")) {
        end += 1
      }
      const word = query.slice(index, end).toUpperCase()
      if (frame.isOrdering && ORDER_END_KEYWORDS.has(word)) {
        finishOrderItem(frame, positions)
        frame.isOrdering = false
      }
      if (word === "BY" && frame.lastWord === "ORDER") {
        frame.isOrdering = true
        frame.itemEnd = undefined
        frame.hasNulls = false
      } else {
        frame.hasNulls = frame.hasNulls || word === "NULLS"
        markOrderFrame(frame, end, word)
      }
      index = end
    } else {
      markOrderFrame(frame, index + 1, undefined)
      index += 1
    }
  }

  for (const frame of frames) {
    finishOrderItem(frame, positions)
  }

  for (const position of positions.sort((left, right) => right - left)) {
    query = `${query.slice(0, position)} NULLS LAST${query.slice(position)}`
  }
  return query
}

type OrderFrame = {
  isOrdering: boolean
  hasNulls: boolean
  itemEnd?: number
  lastWord?: string
}

function createOrderFrame(): OrderFrame {
  return { isOrdering: false, hasNulls: false }
}

function markOrderFrame(frame: OrderFrame, end: number, word: string | undefined) {
  if (frame.isOrdering) frame.itemEnd = end
  frame.lastWord = word
}

function finishOrderItem(frame: OrderFrame, positions: number[]) {
  if (frame.isOrdering && frame.itemEnd !== undefined && !frame.hasNulls) {
    positions.push(frame.itemEnd)
  }
  frame.itemEnd = undefined
  frame.hasNulls = false
}

function isWordChar(char: string) {
  return /^[\p{L}\p{N}_]$/u.test(char)
}

function skipQuoted(query: string, index: number) {
  const quote = query[index] ?? ""
  index += 1
  while (index < query.length) {
    if (query[index] === quote) {
      if (query.startsWith(quote + quote, index)) {
        index += 2
        continue
      }
      return index + 1
    }
    index += 1
  }
  return index
}
