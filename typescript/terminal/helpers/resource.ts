import { inferResourceName, loadDataset } from "@fairspec/library"
import type { Session } from "../session.ts"

export async function selectResource(
  session: Session,
  options: { dataset?: string; resource?: string },
) {
  const dataset = await session.task("Loading dataset", async () => {
    if (!options.dataset) {
      throw new Error("Please provide a path argument or a dataset option")
    }

    return await loadDataset(options.dataset)
  })

  const resource = await session.task("Selecting resource", async () => {
    if (!options.resource) {
      throw new Error("Please provide a resource option")
    }

    const resource = dataset.resources?.find(
      res => options.resource === (res.name ?? inferResourceName(res)),
    )

    if (!resource) {
      throw new Error(`Resource "${options.resource}" not found`)
    }

    return resource
  })

  return resource
}
