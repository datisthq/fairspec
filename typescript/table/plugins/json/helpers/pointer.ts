/**
 * Split an RFC 6901 JSON Pointer into its unescaped reference tokens
 */
export function parseJsonPointer(pointer: string) {
  if (pointer === "") return []
  if (!pointer.startsWith("/")) {
    throw new Error(`Invalid JSON Pointer: ${pointer}`)
  }

  return pointer
    .slice(1)
    .split("/")
    .map(token => token.replaceAll("~1", "/").replaceAll("~0", "~"))
}

/**
 * Join reference tokens into an RFC 6901 JSON Pointer
 */
export function formatJsonPointer(tokens: string[]) {
  return tokens
    .map(token => `/${token.replaceAll("~", "~0").replaceAll("/", "~1")}`)
    .join("")
}
