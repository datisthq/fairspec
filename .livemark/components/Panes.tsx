import { ChevronRight, Wrench } from "lucide-react"
import type { ReactNode } from "react"
import { tk } from "./Landing.tsx"

// Every output below was produced by the real CLIs and libraries on the sample files;
// rerun them after changing a command, a sample or a report message.

export function Pane(props: { label: string; children: ReactNode }) {
  return (
    <div className="p-5 font-mono text-[13px] leading-relaxed lg:p-[2.5cqw] lg:text-[1.1cqw]">
      <p className="mb-4 text-xs text-muted-foreground lg:mb-[1.8cqw] lg:text-[1cqw]">
        {props.label}
      </p>
      <div className="overflow-x-auto">{props.children}</div>
    </div>
  )
}

export function Shell(props: { command: string }) {
  const parts = props.command.match(/"[^"]*"|\s+|\S+/g) ?? []
  return (
    <div className="flex items-start gap-2 whitespace-pre">
      <span className="text-primary shrink-0 select-none">$</span>
      <code className={tk.body}>
        {parts.map((part, i) => (
          <span key={i} className={shellTokenClass(part, i)}>
            {part}
          </span>
        ))}
      </code>
    </div>
  )
}

export function Output(props: { children: ReactNode }) {
  return (
    <pre className="mt-[1.2em]">
      <code className={tk.body}>{props.children}</code>
    </pre>
  )
}

export function InferPane() {
  return (
    <Pane label="terminal">
      <Shell command="fairspec table infer-schema members.csv --json" />
      <Output>
        <TableSchemaSample />
      </Output>
    </Pane>
  )
}

export function ValidatePane() {
  return (
    <Pane label="terminal">
      <Shell command="fairspec table validate broken.csv --schema schema.json" />
      <Output>
        <ValidationErrors />
      </Output>
    </Pane>
  )
}

export function PreviewPane() {
  return (
    <Pane label="terminal">
      <Shell command="fairspec table preview members.csv" />
      <Output>
        <span className={tk.dim}>shape: (3, 4)</span>
        {"\n"}
        {PREVIEW_TABLE}
      </Output>
    </Pane>
  )
}

export function QueryPane() {
  return (
    <Pane label="terminal">
      <Shell command='fairspec table query members.csv "SELECT name, score FROM self WHERE score > 80 ORDER BY score DESC"' />
      <Output>
        <span className={tk.dim}>shape: (2, 2)</span>
        {"\n"}
        {QUERY_TABLE}
      </Output>
    </Pane>
  )
}

export function PythonPane() {
  return (
    <Pane label="validate.py">
      <pre>
        <code className={tk.body}>
          <span className={tk.cmd}>from</span> fairspec{" "}
          <span className={tk.cmd}>import</span> Resource, validate_table
          {"\n\n"}
          resource <span className={tk.punct}>=</span> Resource(data
          <span className={tk.punct}>=</span>
          <span className={tk.str}>"broken.csv"</span>, tableSchema
          <span className={tk.punct}>=</span>
          <span className={tk.str}>"schema.json"</span>){"\n"}
          report <span className={tk.punct}>=</span> validate_table(resource)
          {"\n\n"}
          <span className={tk.key}>print</span>(report.valid)
          {"\n"}
          <span className={tk.dim}># False</span>
          {"\n"}
          <span className={tk.cmd}>for</span> error <span className={tk.cmd}>in</span>{" "}
          report.errors:
          {"\n    "}
          <span className={tk.key}>print</span>(error.type, error.columnName,
          error.rowNumber)
          {"\n"}
          <span className={tk.dim}># cell/type id 3</span>
          {"\n"}
          <span className={tk.dim}># cell/type joined 2</span>
        </code>
      </pre>
    </Pane>
  )
}

export function TypeScriptPane() {
  return (
    <Pane label="validate.ts">
      <pre>
        <code className={tk.body}>
          <span className={tk.cmd}>import</span> {"{ validateTable }"}{" "}
          <span className={tk.cmd}>from</span> <span className={tk.str}>"fairspec"</span>
          {"\n\n"}
          <span className={tk.cmd}>const</span> report <span className={tk.punct}>=</span>{" "}
          <span className={tk.cmd}>await</span>{" "}
          <span className={tk.key}>validateTable</span>({"{"}
          {"\n  "}data: <span className={tk.str}>"broken.csv"</span>,{"\n  "}
          tableSchema: <span className={tk.str}>"schema.json"</span>,{"\n"}
          {"})"}
          {"\n\n"}
          console.<span className={tk.key}>log</span>(report.valid)
          {"\n"}
          <span className={tk.dim}>// false</span>
          {"\n"}
          <span className={tk.cmd}>for</span> (<span className={tk.cmd}>const</span> error{" "}
          <span className={tk.cmd}>of</span> report.errors) {"{"}
          {"\n  "}console.<span className={tk.key}>log</span>(error.type,
          error.columnName, error.rowNumber)
          {"\n"}
          {"}"}
          {"\n"}
          <span className={tk.dim}>// cell/type id 3</span>
          {"\n"}
          <span className={tk.dim}>// cell/type joined 2</span>
        </code>
      </pre>
    </Pane>
  )
}

export function ChatPane(props: { prompt: string; tool: string; children: ReactNode }) {
  return (
    <Pane label="AI assistant">
      <div className="space-y-[0.6em] whitespace-normal">
        <div className="flex items-start gap-2">
          <ChevronRight className="size-[1.1em] mt-[0.25em] text-primary shrink-0" />
          <span className="min-w-0 text-foreground">{props.prompt}</span>
        </div>
        <span className="inline-flex items-center gap-1.5 rounded-md border border-border bg-muted/60 px-[0.6em] py-[0.3em] text-[0.85em] text-muted-foreground">
          <Wrench className="size-[0.9em] text-primary" />
          {props.tool}
        </span>
        <pre className="rounded-md border border-border bg-background px-[0.9em] py-[0.6em] overflow-x-auto">
          <code className={tk.body}>{props.children}</code>
        </pre>
      </div>
    </Pane>
  )
}

export function SkillsPane() {
  return (
    <Pane label="terminal">
      <Shell command="npx skills add https://fairspec.org" />
      <Output>
        <span className={tk.dim}># then ask your agent</span>
        {"\n"}
        <span className={tk.key}>{">"}</span> Describe the CSV files in data/ as a{"\n"}
        {"  "}Fairspec dataset and validate it.
        {"\n\n"}
        <span className={tk.dim}># the agent loads</span>
        {"\n"}
        <span className={tk.str}>fairspec-inference</span>
        {"\n"}
        <span className={tk.str}>fairspec-validation</span>
      </Output>
    </Pane>
  )
}

export function DesktopPane() {
  return (
    <img
      src="/application.webp"
      alt="Fairspec Desktop showing its dataset, table, data and file tools"
      decoding="async"
      className="block aspect-[16/10] w-full object-cover object-top"
    />
  )
}

export function TableSchemaSample() {
  return (
    <>
      <span className={tk.punct}>{"{"}</span>
      {"\n  "}
      <span className={tk.key}>"properties"</span>
      <span className={tk.punct}>: {"{"}</span>
      {"\n    "}
      <SchemaProperty name="id" type="integer" />
      <span className={tk.punct}>,</span>
      {"\n    "}
      <SchemaProperty name="name" type="string" />
      <span className={tk.punct}>,</span>
      {"\n    "}
      <SchemaProperty name="joined" type="string" format="date" />
      <span className={tk.punct}>,</span>
      {"\n    "}
      <SchemaProperty name="score" type="number" />
      {"\n  "}
      <span className={tk.punct}>{"}"}</span>
      {"\n"}
      <span className={tk.punct}>{"}"}</span>
    </>
  )
}

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

export function ValidationReport() {
  return (
    <>
      {"{"}
      {"\n  "}
      <span className={tk.key}>"valid"</span>: <span className={tk.num}>false</span>,
      {"\n  "}
      <span className={tk.key}>"errors"</span>: [
      <ReportError column="id" row={3} cell="x" type="integer" />,
      <ReportError column="joined" row={2} cell="2024-02-30" type="date" />
      {"\n  "}]{"\n"}
      {"}"}
    </>
  )
}

export function QueryRecords() {
  return (
    <>
      {"{"}
      {"\n  "}
      <span className={tk.key}>"records"</span>: [{"\n    "}
      {"{ "}
      <span className={tk.key}>"name"</span>: <span className={tk.str}>"Ada"</span>,{" "}
      <span className={tk.key}>"score"</span>: <span className={tk.num}>91.5</span>
      {" }"},{"\n    "}
      {"{ "}
      <span className={tk.key}>"name"</span>: <span className={tk.str}>"Grace"</span>,{" "}
      <span className={tk.key}>"score"</span>: <span className={tk.num}>88</span>
      {" }"}
      {"\n  "}],{"\n  "}
      <span className={tk.key}>"truncated"</span>: <span className={tk.num}>false</span>
      {"\n"}
      {"}"}
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

function ReportError(props: { column: string; row: number; cell: string; type: string }) {
  return (
    <>
      {"\n    { "}
      <span className={tk.key}>"type"</span>: <span className={tk.str}>"cell/type"</span>,{" "}
      <span className={tk.key}>"columnName"</span>:{" "}
      <span className={tk.str}>"{props.column}"</span>,{" "}
      <span className={tk.key}>"rowNumber"</span>:{" "}
      <span className={tk.num}>{props.row}</span>,{"\n      "}
      <span className={tk.key}>"cell"</span>:{" "}
      <span className={tk.str}>"{props.cell}"</span>,{" "}
      <span className={tk.key}>"columnType"</span>:{" "}
      <span className={tk.str}>"{props.type}"</span>
      {" }"}
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

function shellTokenClass(part: string, index: number) {
  if (index === 0) return tk.cmd
  if (part.startsWith('"')) return tk.str
  if (part.startsWith("-")) return tk.key
  if (part === ">") return tk.punct
  return undefined
}

const PREVIEW_TABLE = `┌─────┬───────┬────────────┬───────┐
│ id  ┆ name  ┆ joined     ┆ score │
│ --- ┆ ---   ┆ ---        ┆ ---   │
│ i64 ┆ str   ┆ date       ┆ f64   │
╞═════╪═══════╪════════════╪═══════╡
│ 1   ┆ Ada   ┆ 2024-01-15 ┆ 91.5  │
│ 2   ┆ Grace ┆ 2024-02-03 ┆ 88.0  │
│ 3   ┆ Linus ┆ 2024-03-21 ┆ 79.25 │
└─────┴───────┴────────────┴───────┘`

const QUERY_TABLE = `┌───────┬───────┐
│ name  ┆ score │
│ ---   ┆ ---   │
│ str   ┆ f64   │
╞═══════╪═══════╡
│ Ada   ┆ 91.5  │
│ Grace ┆ 88.0  │
└───────┴───────┘`
