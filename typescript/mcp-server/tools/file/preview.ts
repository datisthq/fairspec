import { previewFile } from "@fairspec/library"
import { z } from "zod"
import { createTool } from "../../helpers/tool.ts"

export const previewFileTool = createTool({
  name: "previewFileTool",
  description:
    "Preview the raw source of a local or remote file as UTF-8 text: the first lines exactly as stored, before any parsing. Use it to see a CSV's delimiter, header and quoting, or a JSON file's structure. Stops at `maxLines` or `maxBytes`, whichever comes first; `truncated` is true when the file continues.",
  inputSchema: z.object({
    path: z.string().describe("Local or remote path of the file"),
    maxLines: z
      .number()
      .int()
      .min(1)
      .max(200)
      .optional()
      .describe("Maximum number of lines (default 20)"),
    maxBytes: z
      .number()
      .int()
      .min(1)
      .max(20_000)
      .optional()
      .describe("Maximum number of bytes (default 10000)"),
  }),
  outputSchema: z.object({
    text: z.string().describe("The raw file source"),
    truncated: z.boolean().describe("Whether the file continues past the preview"),
  }),
  execute: async input => {
    return await previewFile(input.path, {
      maxLines: input.maxLines,
      maxBytes: input.maxBytes,
    })
  },
})
