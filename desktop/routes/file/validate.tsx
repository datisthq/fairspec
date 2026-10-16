import { ValidateFileInput } from "#models/file.ts"
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
import { Field, FieldDescription, FieldGroup, FieldLabel } from "#elements/field.tsx"
import {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput,
  InputGroupSelect,
} from "#elements/input-group.tsx"
import * as icons from "#icons.ts"
import { engineQuery } from "#services/engine.ts"

export const Route = createFileRoute("/file/validate")({
  component: Component,
  head: () => {
    const title = t`Validate File`
    const description = t`Verify a file against an expected checksum to confirm that the bytes you received are the bytes that were published`

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
        <Trans>Validate File</Trans>
      </h1>
      <p>
        <Trans>
          Verify a file against an expected checksum to confirm that the bytes you
          received are the bytes that were published
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

  const FormInput = ValidateFileInput.extend({})
  const form = useAppForm({
    defaultValues: {
      file: "",
      hashType: "md5",
      hashValue: "",
    } as z.infer<typeof FormInput>,
    validators: {
      onSubmit: FormInput,
    },
    onSubmit: async ({ value }) => {
      validateFile.mutate(value)
    },
  })

  const validateFile = useMutation(
    engineQuery.file.validate.mutationOptions({
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
          name="file"
          children={field => <field.FileOrPathField {...help("file")} />}
        />
        <Field>
          <FieldLabel className="text-xl">
            <Trans>Hash</Trans> <span className="text-destructive">*</span>
          </FieldLabel>
          <FieldDescription className="text-base text-inherit">
            <Trans>Select hash algorithm and enter hash value</Trans>
          </FieldDescription>
          <InputGroup>
            <form.Field
              name="hashType"
              children={field => (
                <InputGroupSelect
                  name={field.name}
                  value={field.state.value}
                  onBlur={field.handleBlur}
                  onChange={(e: React.ChangeEvent<HTMLSelectElement>) =>
                    // @ts-expect-error
                    field.handleChange(e.target.value)
                  }
                >
                  <option value="md5">MD5</option>
                  <option value="sha1">SHA1</option>
                  <option value="sha256">SHA256</option>
                  <option value="sha512">SHA512</option>
                </InputGroupSelect>
              )}
            />
            <form.Field
              name="hashValue"
              children={field => (
                <>
                  <InputGroupInput
                    name={field.name}
                    value={field.state.value}
                    onBlur={field.handleBlur}
                    onChange={e => field.handleChange(e.target.value)}
                    placeholder={t`Enter hash value`}
                  />
                  {field.state.value && (
                    <InputGroupAddon align="inline-end">
                      <InputGroupButton
                        onClick={() => field.handleChange("")}
                        variant="ghost"
                        size="icon-xs"
                        aria-label="Clear field"
                      >
                        <icons.Close />
                      </InputGroupButton>
                    </InputGroupAddon>
                  )}
                </>
              )}
            />
          </InputGroup>
        </Field>
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
            pendingTitle={t`Validating File...`}
            successTitle={t`Valid File`}
            errorTitle={error?.message ?? t`Invalid File`}
          />
        }
      >
        {report?.valid === false && <Report report={report} />}
      </Result>
    </form>
  )
}
