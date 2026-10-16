export function getIsObject(value: any): value is Record<string, any> {
  return typeof value === "object" && value !== null && !Array.isArray(value)
}
