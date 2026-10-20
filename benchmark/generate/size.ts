const UNITS: Record<string, number> = { B: 1, KB: 1e3, MB: 1e6, GB: 1e9 }

/** A data size: a row count, or a byte target such as `16GB`. */
export type Size = number | string

export function parseSize(size: Size) {
  if (typeof size === "number") return { rows: size }
  const match = /^(\d+(?:\.\d+)?)(KB|MB|GB)$/.exec(size)
  if (!match) throw new Error(`Invalid size: ${size}`)
  return { bytes: Math.round(Number(match[1]) * (UNITS[match[2] ?? "B"] ?? 1)) }
}

export function formatBytes(bytes: number) {
  for (const unit of ["GB", "MB", "KB"]) {
    const scale = UNITS[unit] ?? 1
    if (bytes >= scale)
      return `${(bytes / scale).toFixed(bytes >= scale * 10 ? 0 : 1)}${unit}`
  }
  return `${bytes}B`
}
