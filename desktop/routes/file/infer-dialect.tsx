import { InferDialectInput } from "#models/dialect.ts"
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

export const Route = createFileRoute("/file/infer-dialect")({
  component: Component,
  head: () => {
    const title = t`Infer File Dialect`
    const description = t`Automatically infer file formats, encoding specifications, and dialect parameters`

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
        <Trans>Infer File Dialect</Trans>
      </h1>
      <p>
        <Trans>
          Automatically infer file formats, encoding specifications, and dialect
          parameters
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
  const [dialect, setDialect] = useState<any>()
  const [statusType, setStatusType] = useState<StatusType | undefined>()

  const FormInput = InferDialectInput.extend({})
  const form = useAppForm({
    defaultValues: {
      file: "",
    } as z.infer<typeof FormInput>,
    validators: {
      onSubmit: FormInput,
    },
    onSubmit: async ({ value }) => {
      inferDialect.mutate(value)
    },
  })

  const inferDialect = useMutation(
    engineQuery.dialect.infer.mutationOptions({
      onMutate: () => {
        setStatusType("pending")
      },
      onSuccess: dialect => {
        setDialect(dialect)
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
      setDialect(undefined)
      setError(undefined)
    }
  }

  const handleDownloadDialect = async () => {
    if (!dialect) return
    const basename = getFileBasename(form.state.values.file)
    await saveJson(dialect, `${basename}.dialect.json`)
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
          name="file"
          children={field => <field.FileOrPathField {...help("file")} />}
        />
        <form.Subscribe
          selector={state => state.values.file}
          children={file => (
            <Button
              size="lg"
              type="submit"
              form="form"
              className="mt-4 w-full text-xl h-12"
              disabled={!file}
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
            pendingTitle={t`Inferring Dialect...`}
            successTitle={t`Dialect Inferred`}
            errorTitle={error?.message ?? t`Failed to Infer Dialect`}
          />
        }
        action={
          dialect && (
            <Button
              size="lg"
              onClick={handleDownloadDialect}
              className="w-full text-xl h-12"
            >
              <Trans>Save</Trans>
            </Button>
          )
        }
      >
        {dialect && <Json value={dialect} />}
      </Result>
    </form>
  )
}
