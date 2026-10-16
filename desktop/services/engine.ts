import { createORPCClient, onError } from "@orpc/client"
import { RPCLink } from "@orpc/client/message-port"
import type { RouterClient } from "@orpc/server"
import { createTanstackQueryUtils } from "@orpc/tanstack-query"
import type { Router } from "#processes/main/engine.ts"
import * as settings from "#settings.ts"

function createEngineService() {
  const { port1: clientPort, port2: serverPort } = new MessageChannel()
  window.postMessage(settings.ENGINE_IPC, "*", [serverPort])
  clientPort.start()

  const link = new RPCLink({
    port: clientPort,
    interceptors: [
      onError(error => {
        console.error(error)
      }),
    ],
  })

  const client: RouterClient<Router> = createORPCClient(link)
  return client
}

export const engine = createEngineService()

export const engineQuery = createTanstackQueryUtils(engine)
