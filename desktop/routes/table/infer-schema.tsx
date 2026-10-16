import { InferTableSchemaInput } from "#models/tableSchema.ts"
import { t } from "@lingui/core/macro"
import { Trans, useLingui } from "@lingui/react/macro"
import { useMutation } from "@tanstack/react-query"
import { createFileRoute } from "@tanstack/react-router"
import { useState } from "react"
import type * as z from "zod"
import { Json } from "#components/common/Json.tsx"
import { useAppForm } from "#components/form/hooks.ts"
import { useHelpFields } from "#helpers/help.ts"
import { Result } from "#components/result/Result.tsx"
import { Status, type StatusType } from "#components/result/Status.tsx"
import { Button } from "#elements/button.tsx"
import { FieldGroup } from "#elements/field.tsx"
import { saveJson } from "#helpers/json.ts"
import { getFileBasename } from "#helpers/path.ts"
import { engineQuery } from "#services/engine.ts"

export const Route = createFileRoute("/table/infer-schema")({
  component: Component,
  head: () => {
    const title = t`Infer Table Schema`
    const description = t`Automatically infer comprehensive table schema definitions with column types and constraints from your tabular data`

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
        <Trans>Infer Table Schema</Trans>
      </h1>
      <p>
        <Trans>
          Automatically infer comprehensive table schema definitions with column types and
          constraints from your tabular data
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
  const [schema, setSchema] = useState<any>()
  const [statusType, setStatusType] = useState<StatusType | undefined>()

  const FormInput = InferTableSchemaInput.extend({})
  const form = useAppForm({
    defaultValues: {
      table: "",
      dialect: "",
    } as z.infer<typeof FormInput>,
    validators: {
      onSubmit: FormInput,
    },
    onSubmit: async ({ value }) => {
      inferSchema.mutate(value)
    },
  })

  const inferSchema = useMutation(
    engineQuery.tableSchema.infer.mutationOptions({
      onMutate: () => {
        setStatusType("pending")
      },
      onSuccess: schema => {
        setSchema(schema)
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
      setSchema(undefined)
      setError(undefined)
    }
  }

  const handleDownloadSchema = async () => {
    if (!schema) return
    const basename = getFileBasename(form.state.values.table)
    await saveJson(schema, `${basename}.schema.json`)
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
              <Trans>Infer</Trans>
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
            pendingTitle={t`Inferring Schema...`}
            successTitle={t`Schema Inferred`}
            errorTitle={error?.message ?? t`Failed to Infer Schema`}
          />
        }
        action={
          schema && (
            <Button
              size="lg"
              onClick={handleDownloadSchema}
              className="w-full text-xl h-12"
            >
              <Trans>Save</Trans>
            </Button>
          )
        }
      >
        {schema && <Json value={schema} />}
      </Result>
    </form>
  )
}
