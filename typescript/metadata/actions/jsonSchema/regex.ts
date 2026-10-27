// A port of jsonschema-regex's is_valid_ecma_regex, which the Rust twin calls: keep the two
// in step rather than switching to native RegExp, whose u-mode rejects escapes like `\-`

type Previous = "nothing" | "atom" | "assertion" | "quantifier"
type GroupKind = "capturing" | "nonCapturing" | "lookaround"
type Frame = { kind: GroupKind; current: string[]; finished: string[] }

class RegexSyntaxError extends Error {}

/**
 * Whether a pattern is a valid ECMA-262 regular expression under the `u` flag, widened with
 * escapes of non-alphanumeric characters
 */
export function isValidEcmaRegex(pattern: string) {
  try {
    new Parser(pattern).parse()
    return true
  } catch (error) {
    if (error instanceof RegexSyntaxError) return false
    throw error
  }
}

class Parser {
  chars: string[]
  position = 0
  previous: Previous = "nothing"
  frames: Frame[] = [createFrame("nonCapturing")]
  capturingGroups = 0
  largestBackreference = 0
  referencedNames: string[] = []
  declaredNames: string[] = []

  constructor(pattern: string) {
    this.chars = Array.from(pattern)
  }

  parse() {
    while (this.peek() !== undefined) {
      const c = this.bump()
      if (c === "|") {
        const frame = this.getFrame()
        frame.finished.push(...frame.current.splice(0))
        this.previous = "nothing"
      } else if (c === "(") {
        this.openGroup()
        this.previous = "nothing"
      } else if (c === ")") {
        this.previous = this.closeGroup()
      } else if (c === "*" || c === "+" || c === "?") {
        this.quantify()
      } else if (c === "{") {
        this.parseBracedQuantifier()
        this.quantify()
      } else if (c === "}" || c === "]") {
        throw new RegexSyntaxError()
      } else if (c === "^" || c === "$") {
        this.previous = "assertion"
      } else if (c === "\\") {
        this.previous = this.parseAtomEscape()
      } else if (c === "[") {
        this.parseClass()
        this.previous = "atom"
      } else {
        this.previous = "atom"
      }
    }
    if (this.frames.length !== 1 || this.largestBackreference > this.capturingGroups) {
      throw new RegexSyntaxError()
    }
    if (this.referencedNames.some(name => !this.declaredNames.includes(name))) {
      throw new RegexSyntaxError()
    }
  }

  peek(offset = 0): string | undefined {
    return this.chars[this.position + offset]
  }

  bump() {
    const c = this.peek()
    if (c === undefined) throw new RegexSyntaxError()
    this.position += 1
    return c
  }

  eat(expected: string) {
    if (this.peek() !== expected) return false
    this.position += 1
    return true
  }

  expect(expected: string) {
    if (!this.eat(expected)) throw new RegexSyntaxError()
  }

  getFrame() {
    const frame = this.frames.at(-1)
    if (!frame) throw new RegexSyntaxError()
    return frame
  }

  quantify() {
    if (this.previous !== "atom") throw new RegexSyntaxError()
    this.eat("?")
    this.previous = "quantifier"
  }

  parseBracedQuantifier() {
    const lower = this.parseDecimalDigits()
    if (this.eat(",") && this.peek() !== "}") {
      const upper = this.parseDecimalDigits()
      if (compareDecimal(lower, upper) > 0) throw new RegexSyntaxError()
    }
    this.expect("}")
  }

  parseDecimalDigits() {
    const start = this.position
    while (isAsciiDigit(this.peek())) this.position += 1
    if (this.position === start) throw new RegexSyntaxError()
    return this.chars.slice(start, this.position).join("")
  }

  openGroup() {
    let kind: GroupKind = "capturing"
    if (this.eat("?")) {
      const c = this.bump()
      if (c === ":") {
        kind = "nonCapturing"
      } else if (c === "=" || c === "!") {
        kind = "lookaround"
      } else if (c === "<") {
        if (this.peek() === "=" || this.peek() === "!") {
          this.position += 1
          kind = "lookaround"
        } else {
          this.declareName(this.parseGroupName())
          this.capturingGroups += 1
        }
      } else {
        this.parseModifiers(c)
        kind = "nonCapturing"
      }
    } else {
      this.capturingGroups += 1
    }
    this.frames.push(createFrame(kind))
  }

  parseModifiers(first: string) {
    const seen = new Set<string>()
    let dashes = 0
    let c = first
    while (c !== ":") {
      if (c === "i" || c === "m" || c === "s") {
        if (seen.has(c)) throw new RegexSyntaxError()
        seen.add(c)
      } else if (c === "-" && dashes === 0) {
        dashes = 1
      } else {
        throw new RegexSyntaxError()
      }
      c = this.bump()
    }
    if (seen.size === 0) throw new RegexSyntaxError()
  }

  closeGroup(): Previous {
    if (this.frames.length === 1) throw new RegexSyntaxError()
    const frame = this.frames.pop()
    if (!frame) throw new RegexSyntaxError()
    this.getFrame().current.push(...frame.current, ...frame.finished)
    return frame.kind === "lookaround" ? "assertion" : "atom"
  }

  declareName(name: string) {
    if (this.frames.some(frame => frame.current.includes(name)))
      throw new RegexSyntaxError()
    this.declaredNames.push(name)
    this.getFrame().current.push(name)
  }

  parseGroupName() {
    let name = ""
    let first = true
    for (;;) {
      let c = this.bump()
      if (c === ">") break
      if (c === "\\") {
        this.expect("u")
        c = String.fromCodePoint(this.parseUnicodeEscape())
        if (isSurrogate(c)) throw new RegexSyntaxError()
      }
      if (!(first ? isIdentifierStart(c) : isIdentifierPart(c)))
        throw new RegexSyntaxError()
      name += c
      first = false
    }
    if (first) throw new RegexSyntaxError()
    return name
  }

  parseAtomEscape(): Previous {
    const c = this.bump()
    if (c === "b" || c === "B") return "assertion"
    if ("dDsSwW".includes(c)) return "atom"
    if (c === "p" || c === "P") {
      this.parseUnicodeProperty()
    } else if (c === "k") {
      this.expect("<")
      this.referencedNames.push(this.parseGroupName())
    } else if (c >= "1" && c <= "9") {
      this.position -= 1
      const number = Number(this.parseDecimalDigits())
      this.largestBackreference = Math.max(this.largestBackreference, number)
    } else {
      this.parseCharacterEscape(c)
    }
    return "atom"
  }

  parseCharacterEscape(c: string) {
    if (c === "f") return 0x0c
    if (c === "n") return 0x0a
    if (c === "r") return 0x0d
    if (c === "t") return 0x09
    if (c === "v") return 0x0b
    if (c === "c") {
      const letter = this.bump()
      if (!/^[A-Za-z]$/.test(letter)) throw new RegexSyntaxError()
      return codePointOf(letter) % 32
    }
    if (c === "0") {
      if (isAsciiDigit(this.peek())) throw new RegexSyntaxError()
      return 0
    }
    if (c === "x") return this.parseHexDigit() * 16 + this.parseHexDigit()
    if (c === "u") return this.parseUnicodeEscape()
    if (!isAlphanumeric(c)) return codePointOf(c)
    throw new RegexSyntaxError()
  }

  parseHexDigit() {
    const c = this.bump()
    if (!/^[0-9A-Fa-f]$/.test(c)) throw new RegexSyntaxError()
    return Number.parseInt(c, 16)
  }

  parseUnicodeEscape() {
    if (this.eat("{")) {
      let value = 0
      let digits = 0
      while (!this.eat("}")) {
        value = Math.min(value * 16 + this.parseHexDigit(), 0x110000)
        digits += 1
      }
      if (digits === 0 || value > 0x10ffff) throw new RegexSyntaxError()
      return value
    }
    const lead = this.parseHex4()
    if (
      lead >= 0xd800 &&
      lead <= 0xdbff &&
      this.peek() === "\\" &&
      this.peek(1) === "u"
    ) {
      const position = this.position
      this.position += 2
      try {
        const trail = this.parseHex4()
        if (trail >= 0xdc00 && trail <= 0xdfff) {
          return 0x10000 + ((lead - 0xd800) << 10) + (trail - 0xdc00)
        }
      } catch (error) {
        if (!(error instanceof RegexSyntaxError)) throw error
      }
      this.position = position
    }
    return lead
  }

  parseHex4() {
    let value = 0
    for (let index = 0; index < 4; index += 1) value = value * 16 + this.parseHexDigit()
    return value
  }

  parseUnicodeProperty() {
    this.expect("{")
    this.parsePropertyWord()
    if (this.eat("=")) this.parsePropertyWord()
    this.expect("}")
  }

  parsePropertyWord() {
    const start = this.position
    while (/^[A-Za-z0-9_]$/.test(this.peek() ?? "")) this.position += 1
    if (this.position === start) throw new RegexSyntaxError()
  }

  parseClass() {
    this.eat("^")
    for (;;) {
      if (this.eat("]")) return
      const start = this.parseClassAtom()
      if (this.peek() === "-" && this.peek(1) !== "]") {
        this.position += 1
        const end = this.parseClassAtom()
        if (start === undefined || end === undefined || start > end)
          throw new RegexSyntaxError()
      }
    }
  }

  parseClassAtom() {
    const c = this.bump()
    if (c !== "\\") return codePointOf(c)
    const escaped = this.bump()
    if (escaped === "b") return 0x08
    if (escaped === "-") return codePointOf("-")
    if ("dDsSwW".includes(escaped)) return undefined
    if (escaped === "p" || escaped === "P") {
      this.parseUnicodeProperty()
      return undefined
    }
    return this.parseCharacterEscape(escaped)
  }
}

function createFrame(kind: GroupKind): Frame {
  return { kind, current: [], finished: [] }
}

function codePointOf(c: string) {
  return c.codePointAt(0) ?? 0
}

function isAsciiDigit(c: string | undefined) {
  return c !== undefined && c >= "0" && c <= "9"
}

function isSurrogate(c: string) {
  const codePoint = codePointOf(c)
  return codePoint >= 0xd800 && codePoint <= 0xdfff
}

function isAlphanumeric(c: string) {
  return /^[\p{Alphabetic}\p{N}]$/u.test(c)
}

function isIdentifierStart(c: string) {
  return /^[\p{Alphabetic}$_]$/u.test(c)
}

function isIdentifierPart(c: string) {
  return /^[\p{Alphabetic}\p{N}$_‌‍]$/u.test(c)
}

function compareDecimal(left: string, right: string) {
  const a = left.replace(/^0+/, "")
  const b = right.replace(/^0+/, "")
  if (a.length !== b.length) return a.length - b.length
  return a < b ? -1 : a > b ? 1 : 0
}
