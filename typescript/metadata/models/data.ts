import { z } from "zod"
import { Path } from "./path.ts"

export const ResourceDataPath = z.union([Path, z.array(Path)])

export const ResourceDataValue = z.union([
  z.record(z.string(), z.unknown()),
  z.array(z.record(z.string(), z.unknown())),
])

export const ResourceData = z.union([ResourceDataPath, ResourceDataValue])

export const Data = z.unknown()

export type ResourceDataPath = z.infer<typeof ResourceDataPath>
export type ResourceDataValue = z.infer<typeof ResourceDataValue>
export type ResourceData = z.infer<typeof ResourceData>
export type Data = z.infer<typeof Data>
