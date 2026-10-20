import type { CallToolResult } from "@modelcontextprotocol/sdk/types.js"
import type { z } from "zod"
import { MAX_RESPONSE_CHARS } from "./limit.ts"

/**
 * Create an MCP tool: `execute` runs it on typed input, `handle` answers a raw tool call.
 */
export function createTool<I extends z.ZodObject, O extends z.ZodObject>(options: {
  name: string
  description: string
  inputSchema: I
  outputSchema: O
  execute: (input: z.output<I>) => Promise<z.output<O>>
}) {
  const { inputSchema, outputSchema, execute } = options

  // A throw reaches the client as an `isError` result carrying its message
  const handle = async (args: unknown): Promise<CallToolResult> => {
    const output = outputSchema.parse(await execute(inputSchema.parse(args)))
    const text = JSON.stringify(output)

    if (text.length > MAX_RESPONSE_CHARS) {
      throw new Error(
        `The response exceeds ${MAX_RESPONSE_CHARS} characters; narrow the input`,
      )
    }

    return { content: [{ type: "text", text }], structuredContent: output }
  }

  return { ...options, handle }
}
