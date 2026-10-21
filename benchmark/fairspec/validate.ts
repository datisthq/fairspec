import { validateTable } from "@fairspec/library"

const started = performance.now()
const [csvPath, schemaPath] = process.argv.slice(2)
const report = await validateTable({ data: csvPath, tableSchema: schemaPath })
const elapsedMs = performance.now() - started
console.log(JSON.stringify({ errorCount: report.errors.length, elapsedMs }))
