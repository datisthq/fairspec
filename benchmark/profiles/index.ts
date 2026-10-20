import { nyc311Profile } from "./nyc311.ts"
import { simpleProfile } from "./simple.ts"
import type { Profile } from "./types.ts"

export const PROFILES: Record<string, Profile> = {
  [simpleProfile.name]: simpleProfile,
  [nyc311Profile.name]: nyc311Profile,
}

export function getProfile(name: string) {
  const profile = PROFILES[name]
  if (!profile) {
    throw new Error(`Unknown profile ${name}; known: ${Object.keys(PROFILES).join(", ")}`)
  }
  return profile
}
