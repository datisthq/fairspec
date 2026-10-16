import { Option } from "commander"

export const json = new Option("--json", "output as JSON")

export const debug = new Option(
  "--debug",
  "Enable debug mode to print exception details to stderr",
)

export const silent = new Option("--silent", "suppress all output except errors")
