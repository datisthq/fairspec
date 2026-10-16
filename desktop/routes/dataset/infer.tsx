import { InferDatasetInput } from "#models/dataset.ts"
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

export const Route = createFileRoute("/dataset/infer")({
  component: Component,
  head: () => {
    const title = t`Infer Dataset`
    const description = t`Automatically infer dataset metadata, table resources, and the overall structure from your raw data files`

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
        <Trans>Infer Dataset</Trans>
      </h1>
      <p>
        <Trans>
          Automatically infer dataset metadata, table resources, and the overall structure
          from your raw data files
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
  const [dataset, setDataset] = useState<any>()
  const [statusType, setStatusType] = useState<StatusType | undefined>()

  const FormInput = InferDatasetInput.extend({})
  const form = useAppForm({
    defaultValues: {
      table: "",
    } as z.infer<typeof FormInput>,
    validators: {
      onSubmit: FormInput,
    },
    onSubmit: async ({ value }) => {
      inferDataset.mutate(value)
    },
  })

  const inferDataset = useMutation(
    engineQuery.dataset.infer.mutationOptions({
      onMutate: () => {
        setStatusType("pending")
      },
      onSuccess: dataset => {
        setDataset(dataset)
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
      setDataset(undefined)
      setError(undefined)
    }
  }

  const handleDownloadDataset = async () => {
    if (!dataset) return
    const basename = getFileBasename(form.state.values.table)
    await saveJson(dataset, `${basename}.dataset.json`)
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
            pendingTitle={t`Inferring Dataset...`}
            successTitle={t`Dataset Inferred`}
            errorTitle={error?.message ?? t`Failed to Infer Dataset`}
          />
        }
        action={
          dataset && (
            <Button
              size="lg"
              onClick={handleDownloadDataset}
              className="w-full text-xl h-12"
            >
              <Trans>Save</Trans>
            </Button>
          )
        }
      >
        {dataset && <Json value={dataset} />}
      </Result>
    </form>
  )
}
