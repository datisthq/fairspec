import type { InferFileDialectOptions } from "@fairspec/dataset"
import { inferIntegrity, inferTextual } from "@fairspec/dataset"
import type { Resource } from "@fairspec/metadata"
import { copyDescriptor, inferResourceName } from "@fairspec/metadata"
import type { InferTableSchemaOptions } from "@fairspec/table"
import { inferDataSchema } from "../../actions/dataSchema/infer.ts"
import { inferFileDialect } from "../../actions/fileDialect/infer.ts"
import { inferTableSchema } from "../../actions/tableSchema/infer.ts"

export type InferResourceOptions = InferFileDialectOptions & InferTableSchemaOptions

export async function inferResource(resource: Resource, options?: InferResourceOptions) {
  resource = copyDescriptor(resource)

  if (!resource.name) {
    resource.name = inferResourceName(resource)
  }

  if (!resource.fileDialect) {
    const dialect = await inferFileDialect(resource)
    if (dialect) {
      resource.fileDialect = dialect
    }
  }

  if (!resource.textual) {
    const textual = await inferTextual(resource)
    if (textual) {
      resource.textual = textual
    }
  }

  if (!resource.integrity) {
    const integrity = await inferIntegrity(resource)
    if (integrity) {
      resource.integrity = integrity
    }
  }

  if (!resource.dataSchema) {
    const dataSchema = await inferDataSchema(resource)
    if (dataSchema) {
      resource.dataSchema = dataSchema
    }
  }

  if (!resource.tableSchema) {
    const tableSchema = await inferTableSchema(resource, options)
    if (tableSchema) {
      resource.tableSchema = tableSchema
    }
  }

  return resource
}
