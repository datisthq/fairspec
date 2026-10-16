export function calculateTau0(fieldCounts: number[]): number {
  if (fieldCounts.length === 0) return 0

  const stdDev = calculateStdDev(fieldCounts)
  return 1 / (1 + 2 * stdDev)
}

export function calculateTau1(fieldCounts: number[], modalCount: number): number {
  if (fieldCounts.length === 0) return 0

  const rangeScore = calculateRangeScore(fieldCounts, modalCount)
  const transitionScore = calculateTransitionScore(fieldCounts)
  const modeScore = calculateModeScore(fieldCounts, modalCount)

  return (rangeScore + transitionScore + modeScore) / 3
}

function calculateStdDev(counts: number[]): number {
  if (counts.length === 0) return 0

  const mean = counts.reduce((sum, count) => sum + count, 0) / counts.length
  const variance =
    counts.reduce((sum, count) => sum + (count - mean) ** 2, 0) / counts.length

  return Math.sqrt(variance)
}

function calculateRangeScore(counts: number[], modalCount: number): number {
  if (counts.length === 0) return 0

  const min = Math.min(...counts)
  const max = Math.max(...counts)
  const range = max - min

  if (range === 0) return 1
  if (modalCount <= 0) return 0

  const normalizedRange = range / modalCount
  return 1 / (1 + normalizedRange)
}

function calculateTransitionScore(counts: number[]): number {
  if (counts.length <= 1) return 1

  let transitions = 0
  for (let i = 1; i < counts.length; i++) {
    if (counts[i] !== counts[i - 1]) {
      transitions++
    }
  }

  const maxTransitions = counts.length - 1
  const transitionRate = transitions / maxTransitions

  return 1 - transitionRate
}

function calculateModeScore(counts: number[], modalCount: number): number {
  if (counts.length === 0) return 0

  const modalFrequency = counts.filter(count => count === modalCount).length
  const modeRatio = modalFrequency / counts.length

  return modeRatio
}
