import { cn } from "cn"
import type { ReactNode } from "react"
import { type Shot, tk } from "./Landing.tsx"

// Captured from the built app on the files in .agents/skills/landing-terminals/samples/
export const desktopShots = {
  main: desktopShot("main", "Fairspec Desktop's home screen listing its four tools"),
  dataset: desktopShot(
    "dataset",
    "Fairspec Desktop ready to infer a dataset from members.csv",
  ),
  table: desktopShot(
    "table",
    "Fairspec Desktop reporting two invalid cells in broken.csv",
  ),
  data: desktopShot("data", "Fairspec Desktop inferring a data schema from survey.json"),
  file: desktopShot("file", "Fairspec Desktop ready to infer the dialect of members.csv"),
} satisfies Record<string, Shot>

// Outputs are copied from real runs on .agents/skills/landing-terminals/samples/
export function TerminalPane(props: { className?: string; children: ReactNode }) {
  return (
    <div
      className={cn(
        tk.body,
        "space-y-4 font-mono leading-relaxed [overflow-wrap:anywhere]",
        props.className ?? "p-5 text-sm",
      )}
    >
      {props.children}
    </div>
  )
}

// From xl the hero window bleeds off the right edge, so the text keeps a max width.
export function HeroTerminal(props: { children: ReactNode }) {
  return (
    <TerminalPane className="px-3 py-4 text-[10.5px] sm:p-5 sm:text-sm xl:max-w-[42rem] xl:p-8 xl:text-base">
      {props.children}
    </TerminalPane>
  )
}

export function ValidateSession() {
  return (
    <>
      <div>
        <Command args="table validate broken.csv" flags="--schema" rest="schema.json" />
        <ErrorLines />
      </div>
      <div>
        <Command
          args="table validate broken.csv"
          flags="--schema"
          rest="schema.json"
          pipe="'.errors[]'"
        />
        <Json text='{"type":"cell/type","columnName":"id","rowNumber":3,"cell":"x","columnType":"integer"}' />
        <Json text='{"type":"cell/type","columnName":"joined","rowNumber":2,"cell":"2024-02-30","columnType":"date"}' />
      </div>
    </>
  )
}

export function InferSession() {
  return (
    <>
      <div>
        <Command
          args="table infer-schema members.csv"
          flags="--json"
          pipe=".properties"
        />
        <Json text='{"id":{"type":"integer"},"name":{"type":"string"},"joined":{"type":"string","format":"date"},"score":{"type":"number"}}' />
      </div>
      <div>
        <Command args="table validate members.csv" flags="--schema" rest="schema.json" />
        <p>
          <span className={tk.str}>✔</span> Validation passed
        </p>
      </div>
    </>
  )
}

export function PreviewSession() {
  return (
    <div>
      <Command args="table preview members.csv" />
      <TableOutput>
        {`shape: (3, 4)
┌─────┬───────┬────────────┬───────┐
│ id  ┆ name  ┆ joined     ┆ score │
│ --- ┆ ---   ┆ ---        ┆ ---   │
│ i64 ┆ str   ┆ date       ┆ f64   │
╞═════╪═══════╪════════════╪═══════╡
│ 1   ┆ Ada   ┆ 2024-01-15 ┆ 91.5  │
│ 2   ┆ Grace ┆ 2024-02-03 ┆ 88.0  │
│ 3   ┆ Linus ┆ 2024-03-21 ┆ 79.25 │
└─────┴───────┴────────────┴───────┘`}
      </TableOutput>
    </div>
  )
}

export function QuerySession() {
  return (
    <div>
      <Command
        args="table query members.csv"
        sql='"SELECT name, score FROM self WHERE score > 80"'
      />
      <TableOutput>
        {`shape: (2, 2)
┌───────┬───────┐
│ name  ┆ score │
│ ---   ┆ ---   │
│ str   ┆ f64   │
╞═══════╪═══════╡
│ Ada   ┆ 91.5  │
│ Grace ┆ 88.0  │
└───────┴───────┘`}
      </TableOutput>
    </div>
  )
}

export function McpValidateSession() {
  return (
    <>
      <Ask>Check broken.csv against schema.json.</Ask>
      <Step say="Validating the table with the Fairspec MCP server.">
        <ToolCall
          tool="validateTableTool"
          args='{"resource":{"data":"broken.csv","tableSchema":"schema.json"}}'
          result='{"report":{"valid":false,"errors":[{"type":"cell/type","columnName":"id","rowNumber":3,"cell":"x","columnType":"integer"},{"type":"cell/type","columnName":"joined","rowNumber":2,"cell":"2024-02-30","columnType":"date"}]},"truncated":false}'
        />
      </Step>
      <Step say="Two cells fail: id is not an integer in row 3, and 2024-02-30 is not a date in row 2." />
    </>
  )
}

export function McpInferSession() {
  return (
    <>
      <Ask>Describe the columns of members.csv.</Ask>
      <Step say="Inferring a table schema from the file.">
        <ToolCall
          tool="inferTableSchemaTool"
          args='{"resource":{"data":"members.csv"}}'
          result='{"tableSchema":{"properties":{"id":{"type":"integer"},"name":{"type":"string"},"joined":{"type":"string","format":"date"},"score":{"type":"number"}}}}'
        />
      </Step>
      <Step say="Four columns: an integer id, a name, a joined date and a numeric score." />
    </>
  )
}

export function McpQuerySession() {
  return (
    <>
      <Ask>Who in members.csv scored above 80?</Ask>
      <Step say="Querying the table with SQL.">
        <ToolCall
          tool="queryTableTool"
          args='{"resource":{"data":"members.csv"},"query":"SELECT name, score FROM self WHERE score > 80"}'
          result='{"records":[{"name":"Ada","score":91.5},{"name":"Grace","score":88}],"truncated":false}'
        />
      </Step>
      <Step say="Ada with 91.5 and Grace with 88." />
    </>
  )
}

export function SkillsSession() {
  return (
    <>
      <Ask>Why does broken.csv fail its schema?</Ask>
      <Step say="Loading the fairspec-validation skill.">
        <p>
          <span className={cn(tk.key, "font-semibold")}>Skill</span>
          <span className={tk.dim}>(</span>
          <span className={tk.url}>fairspec-validation</span>
          <span className={tk.dim}>)</span>
        </p>
      </Step>
      <Step say="Running the check it recommends.">
        <Command args="table validate broken.csv" flags="--schema" rest="schema.json" />
        <Result>
          <ErrorLines />
        </Result>
      </Step>
      <Step say="Both are cell/type errors: fix the value, or the schema's type or format." />
    </>
  )
}

function Command(props: {
  args: string
  flags?: string
  rest?: string
  sql?: string
  pipe?: string
}) {
  return (
    <p>
      <span className="font-semibold text-destructive">$</span>{" "}
      <span className={cn(tk.key, "font-semibold")}>fairspec</span> {props.args}
      {props.flags ? <span className={tk.url}> {props.flags}</span> : null}
      {props.rest ? ` ${props.rest}` : null}
      {props.sql ? <span className={tk.str}> {props.sql}</span> : null}
      {props.pipe ? (
        <>
          {props.flags === "--json" ? null : <span className={tk.url}> --json</span>}
          <span className={tk.dim}> | </span>
          <span className={tk.key}>jq</span>
          <span className={tk.url}> -c</span>
          <span className={tk.str}> {props.pipe}</span>
        </>
      ) : null}
    </p>
  )
}

function ErrorLines() {
  return (
    <>
      <p>
        <span className="text-destructive">✖</span> Value of the cell x in column id of
        row 3 is not integer
      </p>
      <p>
        <span className="text-destructive">✖</span> Value of the cell 2024-02-30 in column
        joined of row 2 is not date
      </p>
    </>
  )
}

function TableOutput(props: { children: string }) {
  return <pre className="overflow-x-auto leading-[1.2]">{props.children}</pre>
}

function Ask(props: { children: ReactNode }) {
  return (
    <p className="font-semibold">
      <span className={tk.cmd}>›</span> {props.children}
    </p>
  )
}

function Step(props: { say: string; children?: ReactNode }) {
  return (
    <div className="flex gap-2">
      <span className={tk.num}>●</span>
      <div className="min-w-0">
        <p>{props.say}</p>
        {props.children}
      </div>
    </div>
  )
}

function ToolCall(props: { tool: string; args: string; result: string }) {
  return (
    <>
      <p>
        <span className={cn(tk.key, "font-semibold")}>fairspec</span>
        <span className={tk.dim}> · </span>
        <span className={tk.url}>{props.tool}</span>
      </p>
      <div className="pl-[2ch]">
        <Json text={props.args} />
      </div>
      <Result>
        <Json text={props.result} />
      </Result>
    </>
  )
}

function Result(props: { children: ReactNode }) {
  return (
    <div className="flex gap-[1ch]">
      <span className={tk.dim}>⎿</span>
      <div className="min-w-0">{props.children}</div>
    </div>
  )
}

function Json(props: { text: string }) {
  const parts: ReactNode[] = []
  let last = 0
  for (const m of props.text.matchAll(JSON_TOKEN)) {
    if (m.index > last)
      parts.push(
        <span key={last} className={tk.punct}>
          {props.text.slice(last, m.index)}
        </span>,
      )
    const className = m[1] ? tk.key : m[2] ? tk.str : tk.num
    parts.push(
      <span key={m.index} className={className}>
        {m[0]}
      </span>,
    )
    last = m.index + m[0].length
  }
  if (last < props.text.length)
    parts.push(
      <span key={last} className={tk.punct}>
        {props.text.slice(last)}
      </span>,
    )
  return <p>{parts}</p>
}

const JSON_TOKEN =
  /("(?:[^"\\]|\\.)*"(?=:))|("(?:[^"\\]|\\.)*")|true|false|null|-?\d+(?:\.\d+)?/g

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

function desktopShot(id: string, alt: string) {
  return { src: `/desktop/${id}.webp`, srcDark: `/desktop/${id}-dark.webp`, alt }
}
