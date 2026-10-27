import type { WktColumn } from "@fairspec/metadata"
import type { ColumnMapping } from "../../../models/column.ts"
import type { Table } from "../../../models/table.ts"
import { inspectTextColumn } from "../-shared/helpers.ts"

export async function inspectWktColumn(
  column: WktColumn,
  table: Table,
  mapping: ColumnMapping,
) {
  return inspectTextColumn(column, table, {
    mapping,
    parse: parseWkt,
  })
}

/**
 * GEOS rejects a line of one point, and a ring of one or two points or not closed in 2D
 */
export function getIsLineValid(points: Point[], isRing: boolean) {
  if (!points.length) return true
  if (!isRing) return points.length >= 2
  const first = points[0]
  const last = points[points.length - 1]
  return (
    points.length >= 3 &&
    first !== undefined &&
    last !== undefined &&
    first[0] === last[0] &&
    first[1] === last[1]
  )
}

export type Point = [number, number]

type Token =
  | { kind: "open" }
  | { kind: "close" }
  | { kind: "comma" }
  | { kind: "number"; value: number }
  | { kind: "word"; value: string }

type Ordinates = "xy" | "xyz" | "xym" | "xyzm"

const ORDINATE_COUNTS: Record<Ordinates, number> = { xy: 2, xyz: 3, xym: 3, xyzm: 4 }
const ORDINATE_SUFFIXES: Record<string, Ordinates> = { Z: "xyz", M: "xym", ZM: "xyzm" }
const ORDINATE_LENGTHS: Record<number, Ordinates> = { 2: "xy", 3: "xyz", 4: "xyzm" }

const GEOMETRY_KINDS = [
  "POINT",
  "LINESTRING",
  "LINEARRING",
  "POLYGON",
  "MULTIPOINT",
  "MULTILINESTRING",
  "MULTIPOLYGON",
  "GEOMETRYCOLLECTION",
]

// A C `strtod` consuming the whole (lowercased) token, as GEOS reads a WKT number
const NUMBER_REGEX = new RegExp(
  "^[+-]?(?:" +
    "(?:[0-9]+(?:\\.[0-9]*)?|\\.[0-9]+)(?:e[+-]?[0-9]+)?" +
    "|0x(?:[0-9a-f]+(?:\\.[0-9a-f]*)?|\\.[0-9a-f]+)(?:p[+-]?[0-9]+)?" +
    "|inf|infinity|nan(?:\\([0-9a-z_]*\\))?" +
    ")$",
)

/**
 * Whether GEOS (shapely's `from_wkt`) reads the text as a linear geometry
 */
function parseWkt(text: string) {
  const reader = new WktReader(tokenizeWkt(text.split("\0")[0] ?? ""))
  return reader.readGeometry() !== undefined && reader.position === reader.tokens.length
}

function tokenizeWkt(text: string) {
  const tokens: Token[] = []
  let rest = text
  while (true) {
    rest = rest.replace(/^[ \t\n\f\r\v]+/, "")
    const char = rest[0]
    if (char === undefined) return tokens
    if (char === "(") tokens.push({ kind: "open" })
    else if (char === ")") tokens.push({ kind: "close" })
    else if (char === ",") tokens.push({ kind: "comma" })
    else {
      const end = rest.search(/[\n\r\t() ,]/)
      const word = end === -1 ? rest : rest.slice(0, end)
      rest = end === -1 ? "" : rest.slice(end)
      const number = parseWktNumber(word)
      tokens.push(
        number === undefined
          ? { kind: "word", value: word.toUpperCase() }
          : { kind: "number", value: number },
      )
      continue
    }
    rest = rest.slice(1)
  }
}

function parseWktNumber(word: string) {
  const lower = word.toLowerCase()
  if (!NUMBER_REGEX.test(lower)) return undefined
  const unsigned = lower.replace(/^[+-]+/, "")
  const sign = lower.startsWith("-") ? -1 : 1
  if (unsigned.startsWith("nan")) return Number.NaN
  if (unsigned.startsWith("inf")) return sign * Number.POSITIVE_INFINITY
  if (unsigned.startsWith("0x")) {
    const [mantissa = "", exponent = "0"] = unsigned.slice(2).split("p")
    const [integer = "", fraction = ""] = mantissa.split(".")
    let value = 0
    for (const digit of integer + fraction)
      value = value * 16 + Number.parseInt(digit, 16)
    const scale = Number(exponent) - 4 * fraction.length
    return sign * value * 2 ** scale
  }
  return Number(lower)
}

class WktReader {
  tokens: Token[]
  position = 0

  constructor(tokens: Token[]) {
    this.tokens = tokens
  }

  nextToken() {
    const token = this.tokens[this.position]
    this.position += 1
    return token
  }

  peekToken() {
    return this.tokens[this.position]
  }

  readGeometry(parent?: Ordinates): Ordinates | undefined {
    const token = this.nextToken()
    if (token?.kind !== "word") return undefined
    const geometryType = splitGeometryType(token.value)
    if (!geometryType) return undefined
    const { kind } = geometryType
    let ordinates = geometryType.ordinates

    const peeked = this.peekToken()
    if (!ordinates && peeked?.kind === "word") {
      ordinates = ORDINATE_SUFFIXES[peeked.value]
      if (ordinates) this.position += 1
    }

    const dimension = { value: ordinates }
    const opener = this.nextToken()
    let isEmpty: boolean
    if (opener?.kind === "word" && opener.value === "EMPTY") isEmpty = true
    else if (opener?.kind === "open") isEmpty = false
    else return undefined

    let actual: Ordinates
    if (isEmpty) {
      actual = ordinates ?? "xy"
    } else {
      let isRead: boolean
      if (kind === "POINT") {
        isRead = this.readCoordinate(dimension) !== undefined && this.expect("close")
      } else if (kind === "LINESTRING") {
        isRead = this.readLine(dimension, false)
      } else if (kind === "LINEARRING") {
        isRead = this.readLine(dimension, true)
      } else if (kind === "POLYGON") {
        isRead = this.readPolygon(dimension)
      } else if (kind === "MULTIPOINT") {
        isRead = this.readMultiPoint(dimension)
      } else if (kind === "MULTILINESTRING") {
        isRead = this.readList(() =>
          this.readEmptyOr(() => this.readLine(dimension, false)),
        )
      } else if (kind === "MULTIPOLYGON") {
        isRead = this.readList(() => this.readEmptyOr(() => this.readPolygon(dimension)))
      } else {
        const children: Ordinates[] = []
        isRead = this.readList(() => {
          const child = this.readGeometry(ordinates)
          if (!child) return false
          children.push(child)
          return true
        })
        dimension.value =
          ordinates ?? children.reduce<Ordinates | undefined>(mergeOrdinates, undefined)
      }
      if (!isRead) return undefined
      actual = dimension.value ?? "xy"
    }

    if (parent && parent !== actual) return undefined
    return actual
  }

  readMultiPoint(dimension: { value?: Ordinates }) {
    const peeked = this.peekToken()
    const isNested =
      peeked?.kind === "open" || (peeked?.kind === "word" && peeked.value === "EMPTY")
    return this.readList(() => {
      if (!isNested) return this.readCoordinate(dimension) !== undefined
      return this.readEmptyOr(
        () => this.readCoordinate(dimension) !== undefined && this.expect("close"),
      )
    })
  }

  readPolygon(dimension: { value?: Ordinates }) {
    return this.readList(() => this.readEmptyOr(() => this.readLine(dimension, true)))
  }

  // Reads `EMPTY` or an opening parenthesis followed by `read`
  readEmptyOr(read: () => boolean) {
    const token = this.nextToken()
    if (token?.kind === "word" && token.value === "EMPTY") return true
    if (token?.kind === "open") return read()
    return false
  }

  // Reads items separated by commas up to the closing parenthesis
  readList(read: () => boolean) {
    while (true) {
      if (!read()) return false
      const token = this.nextToken()
      if (token?.kind === "comma") continue
      return token?.kind === "close"
    }
  }

  readLine(dimension: { value?: Ordinates }, isRing: boolean) {
    const points: Point[] = []
    const isRead = this.readList(() => {
      const point = this.readCoordinate(dimension)
      if (!point) return false
      points.push(point)
      return true
    })
    return isRead && getIsLineValid(points, isRing)
  }

  readCoordinate(dimension: { value?: Ordinates }): Point | undefined {
    const values: number[] = []
    const limit = dimension.value ? ORDINATE_COUNTS[dimension.value] : 4
    let token = this.peekToken()
    while (token?.kind === "number") {
      values.push(token.value)
      this.position += 1
      if (values.length === limit) break
      token = this.peekToken()
    }

    let expected = dimension.value
    if (!expected) {
      expected = ORDINATE_LENGTHS[values.length]
      if (!expected) return undefined
      dimension.value = expected
    }

    const [x, y] = values
    if (values.length !== ORDINATE_COUNTS[expected]) return undefined
    if (x === undefined || y === undefined) return undefined
    return [x, y]
  }

  expect(kind: "close") {
    return this.nextToken()?.kind === kind
  }
}

// The base type and any `Z`/`M`/`ZM` suffix glued to it (`POINTZ`)
function splitGeometryType(word: string) {
  let base = word
  let ordinates: Ordinates | undefined
  if (word.endsWith("ZM")) {
    base = word.slice(0, -2)
    ordinates = "xyzm"
  } else if (word.endsWith("Z")) {
    base = word.slice(0, -1)
    ordinates = "xyz"
  } else if (word.endsWith("M")) {
    base = word.slice(0, -1)
    ordinates = "xym"
  }
  const kind = GEOMETRY_KINDS.find(kind => kind === base)
  return kind ? { kind, ordinates } : undefined
}

function mergeOrdinates(left: Ordinates | undefined, right: Ordinates): Ordinates {
  if (!left) return right
  const hasZ = [left, right].some(item => item === "xyz" || item === "xyzm")
  const hasM = [left, right].some(item => item === "xym" || item === "xyzm")
  if (hasZ && hasM) return "xyzm"
  if (hasZ) return "xyz"
  if (hasM) return "xym"
  return "xy"
}
