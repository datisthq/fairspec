import type { CategoricalColumn } from "@fairspec/metadata"

export function getCategoricalValuesAndLabels(column: CategoricalColumn) {
  const values: (string | number)[] = []
  const labels: string[] = []

  for (const item of column.property.categories ?? []) {
    if (typeof item === "object") {
      values.push(item.value)
      labels.push(item.label)
    } else {
      values.push(item)
      labels.push(item.toString())
    }
  }

  return { values, labels }
}
