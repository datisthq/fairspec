export class SnifferError extends Error {
  cause?: unknown

  constructor(message: string, cause?: unknown) {
    super(message)
    this.name = "SnifferError"
    this.cause = cause
  }
}

export class EncodingError extends SnifferError {
  constructor(message: string, cause?: unknown) {
    super(message, cause)
    this.name = "EncodingError"
  }
}

export class ParseError extends SnifferError {
  constructor(message: string, cause?: unknown) {
    super(message, cause)
    this.name = "ParseError"
  }
}
