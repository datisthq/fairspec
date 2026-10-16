import { access } from "node:fs/promises"

export async function getIsLocalPathExist(path: string) {
  try {
    await access(path)
    return true
  } catch {
    return false
  }
}

export async function assertLocalPathVacant(path: string) {
  const isExist = await getIsLocalPathExist(path)

  if (isExist) {
    throw new Error(`Path "${path}" already exists`)
  }
}
