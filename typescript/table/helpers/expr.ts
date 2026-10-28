import { Buffer } from "node:buffer"
import * as pl from "nodejs-polars"

/**
 * A field of the struct a `struct.withFields` step extends, as Polars' `pl.field`, which
 * nodejs-polars does not export: a step bound as a field is evaluated once, while an
 * expression reused by value is copied into the tree and evaluated per copy
 */
export function createFieldExpr(name: string) {
  return pl.Expr.deserialize(Buffer.from(JSON.stringify({ Field: [name] })), "json")
}
