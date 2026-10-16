import { ValidateDataInput } from "#models/data.ts"
import type * as fairspec from "@fairspec/metadata"
import { t } from "@lingui/core/macro"
import { Trans, useLingui } from "@lingui/react/macro"
import { useMutation } from "@tanstack/react-query"
import { createFileRoute } from "@tanstack/react-router"
import { useState } from "react"
import type * as z from "zod"
import { useAppForm } from "#components/form/hooks.ts"
import { useHelpFields } from "#helpers/help.ts"
import { Report } from "#components/report/Report.tsx"
import { Result } from "#components/result/Result.tsx"
import { Status, type StatusType } from "#components/result/Status.tsx"
import { Button } from "#elements/button.tsx"
import { FieldGroup } from "#elements/field.tsx"
import { engineQuery } from "#services/engine.ts"

export const Route = createFileRoute("/data/validate")({
  component: Component,
  head: () => {
    const title = t`Validate Data`
    const description = t`Validate data quality, check for inconsistencies and errors, and automatically infer comprehensive data schemas from your datasets`

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
        <Trans>Validate Data</Trans>
      </h1>
      <p>
        <Trans>
          Validate data quality, check for inconsistencies and errors, and automatically
          infer comprehensive data schemas from your datasets
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
  const [report, setReport] = useState<fairspec.Report | undefined>()
  const [statusType, setStatusType] = useState<StatusType | undefined>()

  const Form = ValidateDataInput.extend({})
  const form = useAppForm({
    defaultValues: {
      data: "",
      schema: "",
    } as z.infer<typeof Form>,
    validators: {
      onSubmit: Form,
    },
    onSubmit: async ({ value }) => {
      validateData.mutate(value)
    },
  })

  const validateData = useMutation(
    engineQuery.data.validate.mutationOptions({
      onMutate: () => {
        setStatusType("pending")
      },
      onSuccess: report => {
        setReport(report)
        setStatusType(report.valid ? "success" : "error")
      },
      onError: error => {
        setError(error)
        setStatusType("error")
        // TODO: Fix types
        // @ts-expect-error
        if (error.data.report) setReport(error.data.report)
      },
    }),
  )

  const handleDialogOpenChange = (open: boolean) => {
    if (!open) {
      setStatusType(undefined)
      setReport(undefined)
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
          name="data"
          children={field => <field.FileOrPathField {...help("data")} />}
        />
        <form.AppField
          name="schema"
          children={field => <field.FileOrPathField {...help("schema")} />}
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
              <Trans>Validate</Trans>
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
            pendingTitle={t`Validating Data...`}
            successTitle={t`Valid Data`}
            errorTitle={error?.message ?? t`Invalid Data`}
          />
        }
      >
        {report?.valid === false && <Report report={report} />}
      </Result>
    </form>
  )
}
