import type { ReactNode } from "react"
import { type Shot, tk } from "./Landing.tsx"

// Rendered from real CLI and MCP runs by .agents/skills/landing-screenshots/render.ts
export const shots = {
  validate: shot(
    "validate",
    "fairspec table validate reporting two cell type errors in broken.csv",
  ),
  infer: shot(
    "infer",
    "fairspec inferring a table schema from members.csv, then validating against it",
  ),
  preview: shot(
    "preview",
    "fairspec table preview printing the rows and column types of members.csv",
  ),
  query: shot("query", "fairspec table query running SQL over members.csv"),
  mcpValidate: shot(
    "mcp-validate",
    "An AI assistant calling validateTableTool and summarising the report",
  ),
  mcpInfer: shot(
    "mcp-infer",
    "An AI assistant calling inferTableSchemaTool on members.csv",
  ),
  mcpQuery: shot("mcp-query", "An AI assistant calling queryTableTool with SQL"),
  skills: shot(
    "skills",
    "A coding agent loading the fairspec-validation skill and running the check",
  ),
  desktop: {
    src: "/application.webp",
    srcDark: "/application.webp",
    alt: "Fairspec Desktop showing its dataset, table, data and file tools",
  },
} satisfies Record<string, Shot>

export function ValidationErrors() {
  return (
    <>
      <span className="text-destructive">✖</span> Value of the cell x in column id of row
      3 is not integer
      {"\n"}
      <span className="text-destructive">✖</span> Value of the cell 2024-02-30 in column
      joined of row 2 is not date
    </>
  )
}

export function WindowCode(props: { children: ReactNode }) {
  return (
    <pre className="p-5 text-sm leading-relaxed font-mono overflow-x-auto">
      <code className={tk.body}>{props.children}</code>
    </pre>
  )
}

export function InferredDataset() {
  return (
    <>
      {"{"}
      {"\n  "}
      <span className={tk.key}>"resources"</span>: [{"\n    {"}
      {"\n      "}
      <span className={tk.key}>"data"</span>:{" "}
      <span className={tk.str}>"members.csv"</span>,{"\n      "}
      <span className={tk.key}>"name"</span>: <span className={tk.str}>"members"</span>,
      {"\n      "}
      <span className={tk.key}>"textual"</span>: <span className={tk.num}>true</span>,
      {"\n      "}
      <span className={tk.key}>"integrity"</span>: {"{ "}
      <span className={tk.key}>"type"</span>: <span className={tk.str}>"sha256"</span>,{" "}
      <span className={tk.key}>"hash"</span>:{" "}
      <span className={tk.str}>"67c830fe…108a"</span>
      {" }"},{"\n      "}
      <span className={tk.key}>"fileDialect"</span>: {"{"}
      {"\n        "}
      <span className={tk.key}>"format"</span>: <span className={tk.str}>"csv"</span>,{" "}
      <span className={tk.key}>"delimiter"</span>: <span className={tk.str}>","</span>,
      {"\n        "}
      <span className={tk.key}>"lineTerminator"</span>:{" "}
      <span className={tk.str}>"\n"</span>, <span className={tk.key}>"headerRows"</span>:
      [<span className={tk.num}>1</span>]{"\n      },"}
      {"\n      "}
      <span className={tk.key}>"tableSchema"</span>: {"{"}
      {"\n        "}
      <span className={tk.key}>"properties"</span>: {"{"}
      {"\n          "}
      <SchemaProperty name="id" type="integer" />,{"\n          "}
      <SchemaProperty name="name" type="string" />,{"\n          "}
      <SchemaProperty name="joined" type="string" format="date" />,{"\n          "}
      <SchemaProperty name="score" type="number" />
      {"\n        }"}
      {"\n      }"}
      {"\n    }"}
      {"\n  ]"}
      {"\n}"}
    </>
  )
}

export function MembersCsv() {
  return (
    <>
      <span className={tk.key}>id,name,joined,score</span>
      {"\n"}1,Ada,2024-01-15,91.5{"\n"}2,Grace,2024-02-03,88.0{"\n"}
      3,Linus,2024-03-21,79.25
    </>
  )
}

export function BrokenCsv() {
  return (
    <>
      <span className={tk.key}>id,name,joined,score</span>
      {"\n"}1,Ada,2024-01-15,91.5{"\n"}2,Grace,
      <span className="text-destructive">2024-02-30</span>
      ,88.0{"\n"}
      <span className="text-destructive">x</span>,Linus,2024-03-21,79.25
    </>
  )
}

function SchemaProperty(props: { name: string; type: string; format?: string }) {
  return (
    <>
      <span className={tk.key}>"{props.name}"</span>
      <span className={tk.punct}>: {"{ "}</span>
      <span className={tk.key}>"type"</span>
      <span className={tk.punct}>: </span>
      <span className={tk.str}>"{props.type}"</span>
      {props.format ? (
        <>
          <span className={tk.punct}>, </span>
          <span className={tk.key}>"format"</span>
          <span className={tk.punct}>: </span>
          <span className={tk.str}>"{props.format}"</span>
        </>
      ) : null}
      <span className={tk.punct}>{" }"}</span>
    </>
  )
}

function shot(id: string, alt: string) {
  return { src: `/screenshots/${id}.webp`, srcDark: `/screenshots/${id}-dark.webp`, alt }
}
