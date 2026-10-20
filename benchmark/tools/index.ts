import { fairspecPythonTool, fairspecTypescriptTool } from "./fairspec.ts"
import { qsvTool } from "./qsv.ts"
import {
  dataframelyTool,
  frictionlessTool,
  greatExpectationsTool,
  panderaTool,
  pointblankTool,
} from "./rivals.ts"
import type { Tool } from "./types.ts"

export const TOOLS: Tool[] = [
  fairspecPythonTool,
  fairspecTypescriptTool,
  frictionlessTool,
  qsvTool,
  panderaTool,
  dataframelyTool,
  pointblankTool,
  greatExpectationsTool,
]

export function selectTools(names?: string[]) {
  if (!names?.length) return TOOLS
  const unknown = names.filter(name => !TOOLS.some(tool => tool.name === name))
  if (unknown.length) {
    throw new Error(
      `Unknown tools ${unknown.join(", ")}; known: ${TOOLS.map(tool => tool.name).join(", ")}`,
    )
  }
  return TOOLS.filter(tool => names.includes(tool.name))
}
