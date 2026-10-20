import { inferResourceName, loadDataset } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const listDatasetCommand = f
  .command({ description: "List Dataset resources", configureHelp: helpConfiguration })
  .input(z.object({ path: params.descriptorPath, debug: params.debug }))
  .output(z.array(z.string()))
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })

    const dataset = await session.task("Loading dataset", async () => {
      const dataset = await loadDataset(input.path)

      if (!dataset) {
        throw new Error("Could not load dataset")
      }

      return dataset
    })

    const resourceNames = (dataset.resources ?? []).map(
      resource => resource.name ?? inferResourceName(resource),
    )

    return session.renderDataResult(resourceNames)
  })
