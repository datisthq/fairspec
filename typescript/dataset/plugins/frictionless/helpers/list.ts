/**
 * Normalize a Frictionless field list, which may be a single name
 */
export function convertFieldNames(names: string | string[]) {
  return typeof names === "string" ? [names] : names
}
