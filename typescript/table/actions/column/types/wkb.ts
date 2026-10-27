import type { WkbColumn } from "@fairspec/metadata"
import type { ColumnMapping } from "../../../models/column.ts"
import type { Table } from "../../../models/table.ts"
import { inspectTextColumn } from "../-shared/helpers.ts"
import { getIsLineValid, type Point } from "./wkt.ts"

export async function inspectWkbColumn(
  column: WkbColumn,
  table: Table,
  mapping: ColumnMapping,
) {
  return inspectTextColumn(column, table, {
    mapping,
    parse: parseWkb,
  })
}

/**
 * Whether GEOS (shapely's `from_wkb(bytes.fromhex(text))`) reads a linear geometry;
 * bytes after the geometry are ignored, as GEOS does
 */
function parseWkb(text: string) {
  const bytes = decodeHex(text)
  if (!bytes) return false
  return new WkbReader(bytes).readGeometry() !== undefined
}

// Python's `bytes.fromhex`: ASCII whitespace may separate the digit pairs
function decodeHex(text: string) {
  const bytes: number[] = []
  let index = 0
  while (true) {
    while (index < text.length && " \t\n\r\v\f".includes(text.charAt(index))) index += 1
    if (index >= text.length) return new Uint8Array(bytes)
    const high = parseHexDigit(text.charAt(index))
    const low = parseHexDigit(text.charAt(index + 1))
    if (high === undefined || low === undefined) return undefined
    bytes.push(high * 16 + low)
    index += 2
  }
}

function parseHexDigit(char: string) {
  return /^[0-9a-fA-F]$/.test(char) ? Number.parseInt(char, 16) : undefined
}

class WkbReader {
  view: DataView
  position = 0

  constructor(bytes: Uint8Array) {
    this.view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength)
  }

  readGeometry(expectedType?: number): true | undefined {
    const byteOrder = this.readByte()
    if (byteOrder !== 0 && byteOrder !== 1) return undefined
    const isLittleEndian = byteOrder === 1
    const typeInt = this.readU32(isLittleEndian)
    if (typeInt === undefined) return undefined
    const geometryType = (typeInt & 0xffff) % 1000
    const isoTypeRange = Math.floor((typeInt & 0xffff) / 1000)
    const hasZ = (typeInt & 0x80000000) !== 0 || isoTypeRange === 1 || isoTypeRange === 3
    const hasM = (typeInt & 0x40000000) !== 0 || isoTypeRange === 2 || isoTypeRange === 3
    const dimension = 2 + Number(hasZ) + Number(hasM)
    if ((typeInt & 0x20000000) !== 0 && this.readU32(isLittleEndian) === undefined) {
      return undefined
    }
    if (expectedType !== undefined && expectedType !== geometryType) return undefined

    if (geometryType === 1) {
      return this.readPoints(1, dimension, isLittleEndian) ? true : undefined
    }
    if (geometryType === 2) return this.readLine(dimension, isLittleEndian, false)
    if (geometryType === 3) return this.readPolygon(dimension, isLittleEndian)
    if (geometryType >= 4 && geometryType <= 7) {
      const count = this.readU32(isLittleEndian)
      if (count === undefined) return undefined
      const memberType = geometryType !== 7 ? geometryType - 3 : undefined
      for (let index = 0; index < count; index++) {
        if (!this.readGeometry(memberType)) return undefined
      }
      return true
    }
    return undefined
  }

  readPolygon(dimension: number, isLittleEndian: boolean) {
    const count = this.readU32(isLittleEndian)
    if (count === undefined) return undefined
    for (let index = 0; index < count; index++) {
      if (!this.readLine(dimension, isLittleEndian, true)) return undefined
    }
    return true
  }

  readLine(dimension: number, isLittleEndian: boolean, isRing: boolean) {
    const count = this.readU32(isLittleEndian)
    if (count === undefined) return undefined
    const points = this.readPoints(count, dimension, isLittleEndian)
    return points && getIsLineValid(points, isRing) ? true : undefined
  }

  readPoints(count: number, dimension: number, isLittleEndian: boolean) {
    if (count * dimension * 8 > this.view.byteLength - this.position) return undefined
    const points: Point[] = []
    for (let index = 0; index < count; index++) {
      const x = this.view.getFloat64(this.position, isLittleEndian)
      const y = this.view.getFloat64(this.position + 8, isLittleEndian)
      this.position += dimension * 8
      points.push([x, y])
    }
    return points
  }

  readByte() {
    if (this.position >= this.view.byteLength) return undefined
    const byte = this.view.getUint8(this.position)
    this.position += 1
    return byte
  }

  readU32(isLittleEndian: boolean) {
    if (this.position + 4 > this.view.byteLength) return undefined
    const value = this.view.getUint32(this.position, isLittleEndian)
    this.position += 4
    return value
  }
}
