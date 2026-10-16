import { FairspecException } from "@fairspec/library"
import { ORPCError, os } from "@orpc/server"
import { logger } from "#processes/main/logger.ts"

export const errorMiddleware = os.middleware(async ({ next }) => {
  try {
    return await next()
  } catch (exception) {
    const message = exception instanceof Error ? exception.message : String(exception)

    const report = exception instanceof FairspecException ? exception.report : undefined
    if (!report) {
      logger.withError(exception).error(message)
    }

    throw new ORPCError("UNKNOWN", {
      message,
      data: { report },
    })
  }
})
