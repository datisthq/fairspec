export type Random = ReturnType<typeof createRandom>

export function createRandom(seed: number) {
  let state = seed >>> 0

  function generateFloat() {
    state = (state + 0x6d2b79f5) >>> 0
    let value = state
    value = Math.imul(value ^ (value >>> 15), value | 1)
    value ^= value + Math.imul(value ^ (value >>> 7), value | 61)
    return ((value ^ (value >>> 14)) >>> 0) / 4294967296
  }

  return {
    float: generateFloat,
    integer: (min: number, max: number) =>
      min + Math.floor(generateFloat() * (max - min + 1)),
    chance: (probability: number) => generateFloat() < probability,
    pick: <T>(items: readonly T[]) => items[Math.floor(generateFloat() * items.length)],
    weighted: <T>(items: readonly (readonly [T, number])[]) => {
      const total = items.reduce((sum, [, weight]) => sum + weight, 0)
      let target = generateFloat() * total
      for (const [item, weight] of items) {
        target -= weight
        if (target < 0) return item
      }
      return items[items.length - 1][0]
    },
  }
}
