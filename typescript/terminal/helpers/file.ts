import { getDataFirstPath } from "@fairspec/library"
import type { Session } from "../session.ts"
import { selectResource } from "./resource.ts"

export async function selectFile(
  session: Session,
  options: { dataset?: string; resource?: string },
) {
  const resource = await selectResource(session, options)

  const path = await session.task("Selecting file", async () => {
    const firstPath = getDataFirstPath(resource)
    if (!firstPath) {
      throw new Error("Resource does not have files")
    }

    return firstPath
  })

  return path
}
