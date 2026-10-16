import type * as fairspec from "@fairspec/metadata"
import { Trans } from "@lingui/react/macro"
import { Code } from "#components/common/Code.tsx"

export function Error(props: { error: fairspec.FairspecError }) {
  const { error } = props

  switch (error.type) {
    case "cell/const":
      return <CellConstError error={error} />
    case "cell/enum":
      return <CellEnumError error={error} />
    case "cell/exclusiveMaximum":
      return <CellExclusiveMaximumError error={error} />
    case "cell/exclusiveMinimum":
      return <CellExclusiveMinimumError error={error} />
    case "cell/json":
      return <CellJsonError error={error} />
    case "cell/maxItems":
      return <CellMaxItemsError error={error} />
    case "cell/maxLength":
      return <CellMaxLengthError error={error} />
    case "cell/maximum":
      return <CellMaximumError error={error} />
    case "cell/minItems":
      return <CellMinItemsError error={error} />
    case "cell/minLength":
      return <CellMinLengthError error={error} />
    case "cell/minimum":
      return <CellMinimumError error={error} />
    case "cell/multipleOf":
      return <CellMultipleOfError error={error} />
    case "cell/pattern":
      return <CellPatternError error={error} />
    case "cell/missing":
      return <CellMissingError error={error} />
    case "cell/type":
      return <CellTypeError error={error} />
    case "cell/unique":
      return <CellUniqueError error={error} />
    case "column/missing":
      return <ColumnMissingError error={error} />
    case "column/type":
      return <ColumnTypeError error={error} />
    case "data":
      return <DataError error={error} />
    case "file/integrity":
      return <IntegrityError error={error} />
    case "file/textual":
      return <TextualError error={error} />
    case "foreignKey":
      return <ForeignKeyError error={error} />
    case "metadata":
      return <MetadataError error={error} />
    case "resource/missing":
      return <ResourceMissingError error={error} />
    case "resource/type":
      return <ResourceTypeError error={error} />
    case "row/primaryKey":
      return <RowPrimaryKeyError error={error} />
    case "row/uniqueKey":
      return <RowUniqueError error={error} />
  }
}

// Cell Errors

function CellConstError(props: { error: fairspec.CellConstError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const constValue = <Code>{error.const}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>is not allowed value</Trans> {constValue}
      {inResource}
    </div>
  )
}

function CellEnumError(props: { error: fairspec.CellEnumError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const enumValues = <Code>{error.enum.join(", ")}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>is not in the allowed values</Trans>{" "}
      {enumValues}
      {inResource}
    </div>
  )
}

function CellExclusiveMaximumError(props: { error: fairspec.CellExclusiveMaximumError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const maximum = <Code>{error.maximum}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>is greater or equal to</Trans> {maximum}
      {inResource}
    </div>
  )
}

function CellExclusiveMinimumError(props: { error: fairspec.CellExclusiveMinimumError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const minimum = <Code>{error.minimum}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>is less or equal to</Trans> {minimum}
      {inResource}
    </div>
  )
}

function CellJsonError(props: { error: fairspec.CellJsonError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const jsonPointer = <Code>{error.jsonPointer}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>violates JSON schema at</Trans>{" "}
      {jsonPointer}: {error.message}
      {inResource}
    </div>
  )
}

function CellMaxItemsError(props: { error: fairspec.CellMaxItemsError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const maxItems = <Code>{error.maxItems}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>has more than</Trans> {maxItems}{" "}
      <Trans>items</Trans>
      {inResource}
    </div>
  )
}

function CellMaxLengthError(props: { error: fairspec.CellMaxLengthError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const maxLength = <Code>{error.maxLength}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Length of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>is more than</Trans> {maxLength}
      {inResource}
    </div>
  )
}

function CellMaximumError(props: { error: fairspec.CellMaximumError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const maximum = <Code>{error.maximum}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>is more than</Trans> {maximum}
      {inResource}
    </div>
  )
}

function CellMinItemsError(props: { error: fairspec.CellMinItemsError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const minItems = <Code>{error.minItems}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>has less than</Trans> {minItems}{" "}
      <Trans>items</Trans>
      {inResource}
    </div>
  )
}

function CellMinLengthError(props: { error: fairspec.CellMinLengthError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const minLength = <Code>{error.minLength}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Length of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>is less than</Trans> {minLength}
      {inResource}
    </div>
  )
}

function CellMinimumError(props: { error: fairspec.CellMinimumError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const minimum = <Code>{error.minimum}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>is less than</Trans> {minimum}
      {inResource}
    </div>
  )
}

function CellMultipleOfError(props: { error: fairspec.CellMultipleOfError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const multipleOf = <Code>{error.multipleOf}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>is not a multiple of</Trans> {multipleOf}
      {inResource}
    </div>
  )
}

function CellPatternError(props: { error: fairspec.CellPatternError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const pattern = <Code>{error.pattern}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>does not match the</Trans> {pattern}
      {inResource}
    </div>
  )
}

function CellMissingError(props: { error: fairspec.CellMissingError }) {
  const { error } = props

  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>A required cell in column</Trans> {columnName} <Trans>of row</Trans>{" "}
      {rowNumber} <Trans>is missing</Trans>
      {inResource}
    </div>
  )
}

function CellTypeError(props: { error: fairspec.CellTypeError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const columnType = <Code>{error.columnType}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>is not</Trans> {columnType}
      {inResource}
    </div>
  )
}

function CellUniqueError(props: { error: fairspec.CellUniqueError }) {
  const { error } = props

  const cell = <Code>{error.cell}</Code>
  const columnName = <Code>{error.columnName}</Code>
  const rowNumber = <Code>{error.rowNumber}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Value of the cell</Trans> {cell} <Trans>in column</Trans> {columnName}{" "}
      <Trans>of row</Trans> {rowNumber} <Trans>is not unique</Trans>
      {inResource}
    </div>
  )
}

// Column Errors

function ColumnMissingError(props: { error: fairspec.ColumnMissingError }) {
  const { error } = props

  const columnName = <Code>{error.columnName}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Required column</Trans> {columnName} <Trans>is missing</Trans>
      {inResource}
    </div>
  )
}

function ColumnTypeError(props: { error: fairspec.ColumnTypeError }) {
  const { error } = props

  const columnName = <Code>{error.columnName}</Code>
  const expectedColumnType = <Code>{error.expectedColumnType}</Code>
  const actualColumnType = <Code>{error.actualColumnType}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Column</Trans> {columnName} <Trans>is expected to be</Trans>{" "}
      {expectedColumnType}
      <Trans>, but it is</Trans> {actualColumnType}
      {inResource}
    </div>
  )
}

// Data Errors

function DataError(props: { error: fairspec.DataError }) {
  const { error } = props

  const jsonPointer = <Code>{error.jsonPointer}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Data error at</Trans> {jsonPointer}: {error.message}
      {inResource}
    </div>
  )
}

// File Errors

function IntegrityError(props: { error: fairspec.IntegrityError }) {
  const { error } = props

  const hashType = <Code>{error.hashType}</Code>
  const expectedHash = <Code>{error.expectedHash}</Code>
  const actualHash = <Code>{error.actualHash}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>File hash</Trans> {hashType} <Trans>is expected to be</Trans> {expectedHash}
      <Trans>, but it is</Trans> {actualHash}
      {inResource}
    </div>
  )
}

function TextualError(props: { error: fairspec.TextualError }) {
  const { error } = props

  const actualEncoding = <Code>{error.actualEncoding ?? "binary"}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>File is expected to be textual with utf-8 encoding but it is</Trans>{" "}
      {actualEncoding}
      {inResource}
    </div>
  )
}

// ForeignKey Errors

function ForeignKeyError(props: { error: fairspec.ForeignKeyError }) {
  const { error } = props

  const cells = <Code>{error.cells.join(", ")}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Foreign key constraint violated as cells</Trans> {cells}{" "}
      <Trans>do not reference existing values</Trans>
      {inResource}
    </div>
  )
}

// Metadata Errors

function MetadataError(props: { error: fairspec.MetadataError }) {
  const { error } = props

  const jsonPointer = <Code>{error.jsonPointer}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      {error.message} <Trans>at</Trans> {jsonPointer}
      {inResource}
    </div>
  )
}

// Resource Errors

function ResourceMissingError(props: { error: fairspec.ResourceMissingError }) {
  const { error } = props

  const resourceName = <Code>{error.resourceName}</Code>
  const inReferencingResource = (
    <InResource resourceName={error.referencingResourceName} />
  )

  return (
    <div>
      <Trans>Resource</Trans> {resourceName} <Trans>is missing, but expected</Trans>{" "}
      {inReferencingResource}
    </div>
  )
}

function ResourceTypeError(props: { error: fairspec.ResourceTypeError }) {
  const { error } = props

  const resourceName = <Code>{error.resourceName}</Code>
  const expectedResourceType = <Code>{error.expectedResourceType}</Code>
  const inReferencingResource = (
    <InResource resourceName={error.referencingResourceName} />
  )

  return (
    <div>
      <Trans>Resource</Trans> {resourceName} <Trans>is expected to be</Trans>{" "}
      {expectedResourceType}
      {inReferencingResource}
    </div>
  )
}

// Row Errors

function RowPrimaryKeyError(props: { error: fairspec.RowPrimaryKeyError }) {
  const { error } = props

  const rowNumber = <Code>{error.rowNumber}</Code>
  const columnNames = <Code>{error.columnNames.join(", ")}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Row</Trans> {rowNumber}{" "}
      <Trans>violates primary key constraint on columns</Trans> {columnNames}
      {inResource}
    </div>
  )
}

function RowUniqueError(props: { error: fairspec.RowUniqueKeyError }) {
  const { error } = props

  const rowNumber = <Code>{error.rowNumber}</Code>
  const columnNames = <Code>{error.columnNames.join(", ")}</Code>
  const inResource = <InResource resourceName={error.resourceName} />

  return (
    <div>
      <Trans>Row</Trans> {rowNumber}{" "}
      <Trans>violates unique key constraint on columns</Trans> {columnNames}
      {inResource}
    </div>
  )
}

// Internal

function InResource(props: { resourceName: string | undefined }) {
  const { resourceName } = props

  if (!resourceName) {
    return null
  }

  return (
    <span>
      {" "}
      <Trans>in resource</Trans>
      <Code>{resourceName}</Code>
    </span>
  )
}
