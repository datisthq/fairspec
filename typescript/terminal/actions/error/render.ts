import type {
  CellConstError,
  CellEnumError,
  CellExclusiveMaximumError,
  CellExclusiveMinimumError,
  CellJsonError,
  CellMaxItemsError,
  CellMaximumError,
  CellMaxLengthError,
  CellMinItemsError,
  CellMinimumError,
  CellMinLengthError,
  CellMissingError,
  CellMultipleOfError,
  CellPatternError,
  CellTypeError,
  CellUniqueError,
  ColumnMissingError,
  ColumnTypeError,
  DataError,
  FairspecError,
  ForeignKeyError,
  IntegrityError,
  MetadataError,
  ResourceMissingError,
  ResourceTypeError,
  RowPrimaryKeyError,
  RowUniqueKeyError,
  TextualError,
} from "@fairspec/library"
import pc from "picocolors"

export function renderError(error: FairspecError) {
  switch (error.type) {
    case "cell/type":
      return renderCellTypeError(error)
    case "cell/missing":
      return renderCellMissingError(error)
    case "cell/minimum":
      return renderCellMinimumError(error)
    case "cell/maximum":
      return renderCellMaximumError(error)
    case "cell/exclusiveMinimum":
      return renderCellExclusiveMinimumError(error)
    case "cell/exclusiveMaximum":
      return renderCellExclusiveMaximumError(error)
    case "cell/multipleOf":
      return renderCellMultipleOfError(error)
    case "cell/minLength":
      return renderCellMinLengthError(error)
    case "cell/maxLength":
      return renderCellMaxLengthError(error)
    case "cell/pattern":
      return renderCellPatternError(error)
    case "cell/unique":
      return renderCellUniqueError(error)
    case "cell/const":
      return renderCellConstError(error)
    case "cell/enum":
      return renderCellEnumError(error)
    case "cell/json":
      return renderCellJsonError(error)
    case "cell/minItems":
      return renderCellMinItemsError(error)
    case "cell/maxItems":
      return renderCellMaxItemsError(error)
    case "column/missing":
      return renderColumnMissingError(error)
    case "column/type":
      return renderColumnTypeError(error)
    case "data":
      return renderDataError(error)
    case "file/textual":
      return renderTextualError(error)
    case "file/integrity":
      return renderIntegrityError(error)
    case "foreignKey":
      return renderForeignKeyError(error)
    case "metadata":
      return renderMetadataError(error)
    case "row/primaryKey":
      return renderRowPrimaryKeyError(error)
    case "row/uniqueKey":
      return renderRowUniqueKeyError(error)
    case "resource/missing":
      return renderResourceMissingError(error)
    case "resource/type":
      return renderResourceTypeError(error)
  }
}

// Cell

export function renderCellTypeError(error: CellTypeError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const columnType = pc.bold(error.columnType)
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} is not ${columnType}${inResource}`
}

export function renderCellMissingError(error: CellMissingError) {
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const inResource = renderInResource(error.resourceName)

  return `A cell in column ${columnName} of row ${rowNumber} is missing${inResource}`
}

export function renderCellMinimumError(error: CellMinimumError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const minimum = pc.bold(error.minimum)
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} is less than ${minimum}${inResource}`
}

export function renderCellMaximumError(error: CellMaximumError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const maximum = pc.bold(error.maximum)
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} is more than ${maximum}${inResource}`
}

export function renderCellExclusiveMinimumError(error: CellExclusiveMinimumError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const minimum = pc.bold(error.minimum)
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} is less or equal to ${minimum}${inResource}`
}

export function renderCellExclusiveMaximumError(error: CellExclusiveMaximumError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const maximum = pc.bold(error.maximum)
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} is greater or equal to ${maximum}${inResource}`
}

export function renderCellMultipleOfError(error: CellMultipleOfError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const multipleOf = pc.bold(error.multipleOf)
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} is not a multiple of ${multipleOf}${inResource}`
}

export function renderCellMinLengthError(error: CellMinLengthError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const minLength = pc.bold(error.minLength)
  const inResource = renderInResource(error.resourceName)

  return `Length of the cell ${cell} in column ${columnName} of row ${rowNumber} is less than ${minLength}${inResource}`
}

export function renderCellMaxLengthError(error: CellMaxLengthError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const maxLength = pc.bold(error.maxLength)
  const inResource = renderInResource(error.resourceName)

  return `Length of the cell ${cell} in column ${columnName} of row ${rowNumber} is more than ${maxLength}${inResource}`
}

export function renderCellPatternError(error: CellPatternError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const pattern = pc.bold(error.pattern)
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} does not match the ${pattern}${inResource}`
}

export function renderCellUniqueError(error: CellUniqueError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} is not unique${inResource}`
}

export function renderCellConstError(error: CellConstError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const constValue = pc.bold(error.const)
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} is not allowed value ${constValue}${inResource}`
}

export function renderCellEnumError(error: CellEnumError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const enumValues = error.enum.map(v => pc.bold(v)).join(", ")
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} is not in the allowed values ${enumValues}${inResource}`
}

export function renderCellJsonError(error: CellJsonError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const jsonPointer = pc.bold(error.jsonPointer)
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} violates JSON schema at ${jsonPointer}: ${error.message}${inResource}`
}

export function renderCellMinItemsError(error: CellMinItemsError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const minItems = pc.bold(error.minItems)
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} has less than ${minItems} items${inResource}`
}

export function renderCellMaxItemsError(error: CellMaxItemsError) {
  const cell = pc.bold(error.cell)
  const columnName = pc.bold(error.columnName)
  const rowNumber = pc.bold(error.rowNumber)
  const maxItems = pc.bold(error.maxItems)
  const inResource = renderInResource(error.resourceName)

  return `Value of the cell ${cell} in column ${columnName} of row ${rowNumber} has more than ${maxItems} items${inResource}`
}

// Column

export function renderColumnMissingError(error: ColumnMissingError) {
  const columnName = pc.bold(error.columnName)
  const inResource = renderInResource(error.resourceName)

  return `Required column ${columnName} is missing${inResource}`
}

export function renderColumnTypeError(error: ColumnTypeError) {
  const columnName = pc.bold(error.columnName)
  const expectedColumnType = pc.bold(error.expectedColumnType)
  const actualColumnType = pc.bold(error.actualColumnType)
  const inResource = renderInResource(error.resourceName)

  return `Column ${columnName} is expected to be ${expectedColumnType}, but it is ${actualColumnType}${inResource}`
}

// Data

export function renderDataError(error: DataError) {
  const jsonPointer = pc.bold(error.jsonPointer)
  const inResource = renderInResource(error.resourceName)

  return `Data error at ${jsonPointer}: ${error.message}${inResource}`
}

// File

export function renderTextualError(error: TextualError) {
  const actualEncoding = error.actualEncoding ? pc.bold(error.actualEncoding) : ""
  const inResource = renderInResource(error.resourceName)

  return `File is expected to be textual with utf-8 encoding but it is ${actualEncoding ?? "binary"}${inResource}`
}

export function renderIntegrityError(error: IntegrityError) {
  const hashType = pc.bold(error.hashType)
  const expectedHash = pc.bold(error.expectedHash)
  const actualHash = pc.bold(error.actualHash)
  const inResource = renderInResource(error.resourceName)

  return `File hash ${hashType} is expected to be ${expectedHash}, but it is ${actualHash})${inResource}`
}

// Foreign Key

export function renderForeignKeyError(error: ForeignKeyError) {
  const cells = error.cells.map(c => pc.bold(c)).join(", ")
  const inResource = renderInResource(error.resourceName)

  return `Foreign key constraint violated as cells ${cells} do not reference existing values${inResource}`
}

// Metadata

export function renderMetadataError(error: MetadataError) {
  const jsonPointer = pc.bold(error.jsonPointer)
  const inResource = renderInResource(error.resourceName)

  return `${error.message} at ${jsonPointer}${inResource}`
}

// Resource

export function renderResourceMissingError(error: ResourceMissingError) {
  const resourceName = pc.bold(error.resourceName)
  const inReferencingResource = renderInResource(error.referencingResourceName)

  return `Resource ${resourceName} is missing, but expected${inReferencingResource}`
}

export function renderResourceTypeError(error: ResourceTypeError) {
  const resourceName = pc.bold(error.resourceName)
  const expectedResourceType = pc.bold(error.expectedResourceType)
  const inReferencingResource = renderInResource(error.referencingResourceName)

  return `Resource ${resourceName} is expected to be ${expectedResourceType}${inReferencingResource}`
}

// Row

export function renderRowPrimaryKeyError(error: RowPrimaryKeyError) {
  const rowNumber = pc.bold(error.rowNumber)
  const columnNames = error.columnNames.map(c => pc.bold(c)).join(", ")
  const inResource = renderInResource(error.resourceName)

  return `Row ${rowNumber} violates primary key constraint on columns ${columnNames}${inResource}`
}

export function renderRowUniqueKeyError(error: RowUniqueKeyError) {
  const rowNumber = pc.bold(error.rowNumber)
  const columnNames = error.columnNames.map(c => pc.bold(c)).join(", ")
  const inResource = renderInResource(error.resourceName)

  return `Row ${rowNumber} violates unique key constraint on columns ${columnNames}${inResource}`
}

// Internal

function renderInResource(resourceName?: string) {
  return resourceName ? ` in resource ${pc.bold(resourceName)}` : ""
}
