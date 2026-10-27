/**
 * Convert a Frictionless name to a Fairspec resource name (letters, digits and `_`)
 */
export function convertNameFromFrictionless(name: string) {
  return name.replace(/[^a-zA-Z0-9_]/g, "_")
}
