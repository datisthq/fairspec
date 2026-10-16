import { onError } from "@orpc/server"
import { RPCHandler } from "@orpc/server/message-port"
import { inferDataSchemaEndpoint } from "#endpoints/dataSchema/infer.ts"
import { validateDataEndpoint } from "#endpoints/data/validate.ts"
import { inferDatasetEndpoint } from "#endpoints/dataset/infer.ts"
import { validateDatasetEndpoint } from "#endpoints/dataset/validate.ts"
import { inferDialectEndpoint } from "#endpoints/dialect/infer.ts"
import { validateFileEndpoint } from "#endpoints/file/validate.ts"
import { previewTableEndpoint } from "#endpoints/table/preview.ts"
import { validateTableEndpoint } from "#endpoints/table/validate.ts"
import { inferTableSchemaEndpoint } from "#endpoints/tableSchema/infer.ts"
import { logger } from "#processes/main/logger.ts"

export const router = {
  data: {
    validate: validateDataEndpoint,
  },
  dataSchema: {
    infer: inferDataSchemaEndpoint,
  },
  dataset: {
    infer: inferDatasetEndpoint,
    validate: validateDatasetEndpoint,
  },
  dialect: {
    infer: inferDialectEndpoint,
  },
  file: {
    validate: validateFileEndpoint,
  },
  table: {
    preview: previewTableEndpoint,
    validate: validateTableEndpoint,
  },
  tableSchema: {
    infer: inferTableSchemaEndpoint,
  },
}

export type Router = typeof router

export const engineRpcHandler = new RPCHandler(router, {
  interceptors: [
    onError(error => {
      logger.error(String(error))
    }),
  ],
})
