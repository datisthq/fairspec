import type { MetadataError, TableSchema } from "@fairspec/metadata"
import * as pl from "nodejs-polars"

/**
 * Report each column pattern the table engine cannot compile, such as one using look-around
 */
export function inspectTableSchemaPatterns(tableSchema: TableSchema) {
  const errors: MetadataError[] = []

  for (const [name, property] of Object.entries(tableSchema.properties ?? {})) {
    if (!("pattern" in property) || typeof property.pattern !== "string") continue

    try {
      pl.DataFrame({ value: [""] }).select(pl.col("value").str.contains(property.pattern))
    } catch (error) {
      const reason = error instanceof Error ? error.message.trim().split("\n").at(-1) : ""
      errors.push({
        type: "metadata",
        message: `pattern "${property.pattern}" cannot be compiled: ${reason}`,
        jsonPointer: `/properties/${name.replaceAll("~", "~0").replaceAll("/", "~1")}/pattern`,
      })
    }
  }

  return errors
}
