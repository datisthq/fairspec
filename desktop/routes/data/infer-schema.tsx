import { InferDataSchemaInput } from "#models/dataSchema.ts"
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

export const Route = createFileRoute("/data/infer-schema")({
  component: Component,
  head: () => {
    const title = t`Infer Data Schema`
    const description = t`Automatically infer comprehensive data schemas from your datasets`

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
        <Trans>Infer Data Schema</Trans>
      </h1>
      <p>
        <Trans>Automatically infer comprehensive data schemas from your datasets</Trans>.
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

  const Form = InferDataSchemaInput.extend({})
  const form = useAppForm({
    defaultValues: {
      data: "",
    } as z.infer<typeof Form>,
    validators: {
      onSubmit: Form,
    },
    onSubmit: async ({ value }) => {
      inferSchema.mutate(value)
    },
  })

  const inferSchema = useMutation(
    engineQuery.dataSchema.infer.mutationOptions({
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
    const basename = getFileBasename(form.state.values.data)
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
          name="data"
          children={field => <field.FileOrPathField {...help("data")} />}
        />
        <form.Subscribe
          selector={state => state.values.data}
          children={data => (
            <Button
              size="lg"
              type="submit"
              form="form"
              className="mt-4 w-full text-xl h-12"
              disabled={!data}
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
