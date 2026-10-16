// import { FileDialect } from "../models/fileDialect/fileDialect.ts"
// import { Report } from "../models/report.ts"
// import { Resource } from "../models/resource.ts"
import { TableSchema } from "../models/tableSchema.ts"

// console.log(Report.toJSONSchema())
// console.log(Resource.toJSONSchema())
// console.log(FileDialect.toJSONSchema())
// console.log(TableSchema.toJSONSchema())

const schema = TableSchema.parse({
  properties: {
    id: { type: "integer" },
    name: { type: "string" },
    age: { type: "integer" },
  },
})

console.log(schema)
