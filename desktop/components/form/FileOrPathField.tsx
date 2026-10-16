import type { FileType } from "#models/file.ts"
import { Trans } from "@lingui/react/macro"
import { Field, FieldDescription, FieldError, FieldLabel } from "#elements/field.tsx"
import {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput,
} from "#elements/input-group.tsx"
import * as icons from "#icons.ts"
import { useFieldContext } from "./context.ts"

export function FileOrPathField(props: {
  label: string
  fileType: FileType
  description?: string
  placeholder?: string
  required?: boolean
  disabled?: boolean
}) {
  const field = useFieldContext<File | string>()

  const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
  const value = field.state.value
    ? typeof field.state.value === "string"
      ? field.state.value
      : field.state.value.name
    : ""

  const fileInputAccept = props.fileType !== "file" ? [".json"] : undefined
  if (fileInputAccept && props.fileType === "table") {
    fileInputAccept.push(
      ".csv",
      ".tsv",
      ".jsonl",
      ".ndjson",
      ".xlsx",
      ".ods",
      ".parquet",
      ".arrow",
      ".feather",
      ".sqlite",
    )
  }

  const handleFileSelect = async () => {
    const filters = fileInputAccept
      ? [
          {
            name: props.fileType === "table" ? "Table Files" : "JSON Files",
            extensions: fileInputAccept.map(ext => ext.slice(1)),
          },
        ]
      : undefined

    const filePath = await globalThis.desktop.openFileDialog({ filters })
    if (filePath) {
      field.handleChange(filePath)
    }
  }

  return (
    <Field data-invalid={isInvalid}>
      <FieldLabel htmlFor={field.name} className="text-xl">
        {props.label} {props.required && <span className="text-destructive">*</span>}
      </FieldLabel>
      {props.description && (
        <FieldDescription className="text-base text-inherit">
          {props.description}
        </FieldDescription>
      )}
      <InputGroup>
        <InputGroupInput
          name={field.name}
          value={value}
          onBlur={field.handleBlur}
          onChange={e => field.handleChange(e.target.value)}
          aria-invalid={isInvalid}
          placeholder={props.placeholder}
          autoComplete="off"
          disabled={props.disabled}
        />
        <InputGroupAddon align="inline-start">
          <InputGroupButton
            onClick={handleFileSelect}
            variant="secondary"
            disabled={props.disabled}
          >
            <icons.Open />
            <Trans>Open</Trans>
          </InputGroupButton>
        </InputGroupAddon>
        {field.state.value && (
          <InputGroupAddon align="inline-end">
            <InputGroupButton
              onClick={() => field.handleChange("")}
              variant="ghost"
              size="icon-xs"
              aria-label="Clear field"
              disabled={props.disabled}
            >
              <icons.Close />
            </InputGroupButton>
          </InputGroupAddon>
        )}
      </InputGroup>
      {isInvalid && <FieldError errors={field.state.meta.errors} />}
    </Field>
  )
}
