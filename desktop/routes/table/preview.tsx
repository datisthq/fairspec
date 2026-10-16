import { PreviewTableInput } from "#models/table.ts"
import type { TableSchema } from "@fairspec/metadata"
import { t } from "@lingui/core/macro"
import { Trans, useLingui } from "@lingui/react/macro"
import { useMutation } from "@tanstack/react-query"
import { createFileRoute } from "@tanstack/react-router"
import { useState } from "react"
import type * as z from "zod"
import { useAppForm } from "#components/form/hooks.ts"
import { useHelpFields } from "#helpers/help.ts"
import { Result } from "#components/result/Result.tsx"
import { Status, type StatusType } from "#components/result/Status.tsx"
import { Table } from "#components/table/Table.tsx"
import { Button } from "#elements/button.tsx"
import { FieldGroup } from "#elements/field.tsx"
import { engineQuery } from "#services/engine.ts"

export const Route = createFileRoute("/table/preview")({
  component: Component,
  head: () => {
    const title = t`Preview Table`
    const description = t`Load and preview your data table`

    return {
      meta: [
        { title },
        { name: "description", content: description },
        { property: "og:title", content: title },
        { property: "og:description", content: description },
      ],
    }
  },
})

function Component() {
  return (
    <div className="flex flex-col gap-8">
      <Intro />
      <Form />
    </div>
  )
}

function Intro() {
  return (
    <div className="text-content">
      <h1 id="top">
        <Trans>Preview Table</Trans>
      </h1>
      <p>
        <Trans>
          Load tabular data from your files and preview rows, columns, and overall
          structure to inspect contents
        </Trans>
        .
      </p>
    </div>
  )
}

function Form() {
  const help = useHelpFields()
  const { t } = useLingui()
  const [error, setError] = useState<Error | undefined>()
  const [records, setRecords] = useState<Record<string, unknown>[] | undefined>()
  const [tableSchema, setTableSchema] = useState<TableSchema | undefined>()
  const [statusType, setStatusType] = useState<StatusType | undefined>()

  const FormInput = PreviewTableInput.extend({})
  const form = useAppForm({
    defaultValues: {
      table: "",
      schema: "",
      dialect: "",
    } as z.infer<typeof FormInput>,
    validators: {
      onSubmit: FormInput,
    },
    onSubmit: async ({ value }) => {
      previewTable.mutate(value)
    },
  })

  const previewTable = useMutation(
    engineQuery.table.preview.mutationOptions({
      onMutate: () => {
        setStatusType("pending")
      },
      onSuccess: data => {
        setRecords(data.records)
        setTableSchema(data.tableSchema)
        setStatusType("success")
      },
      onError: error => {
        setError(error)
        setStatusType("error")
      },
    }),
  )

  const handleDialogOpenChange = (open: boolean) => {
    if (!open) {
      setStatusType(undefined)
      setRecords(undefined)
      setTableSchema(undefined)
      setError(undefined)
    }
  }

  return (
    <form
      id="form"
      onSubmit={e => {
        e.preventDefault()
        form.handleSubmit()
      }}
    >
      <FieldGroup>
        <form.AppField
          name="table"
          children={field => <field.FileOrPathField {...help("table")} />}
        />
        <form.AppField
          name="schema"
          children={field => <field.FileOrPathField {...help("schema")} />}
        />
        <form.AppField
          name="dialect"
          children={field => <field.FileOrPathField {...help("dialect")} />}
        />
        <form.Subscribe
          selector={state => state.values.table}
          children={table => (
            <Button
              size="lg"
              type="submit"
              form="form"
              className="mt-4 w-full text-xl h-12"
              disabled={!table}
            >
              <Trans>Preview</Trans>
            </Button>
          )}
        />
      </FieldGroup>
      <Result
        open={!!statusType}
        onOpenChange={handleDialogOpenChange}
        status={
          <Status
            statusType={statusType}
            pendingTitle={t`Loading Preview...`}
            successTitle={t`Table Preview`}
            errorTitle={error?.message ?? t`Failed to Load Preview`}
          />
        }
      >
        {records && tableSchema && (
          <Table records={records} tableSchema={tableSchema} className="mt-4" />
        )}
      </Result>
    </form>
  )
}
