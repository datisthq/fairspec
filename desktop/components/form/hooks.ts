import { createFormHook } from "@tanstack/react-form"
import { fieldContext, formContext } from "./context.ts"
import { FileOrPathField } from "./FileOrPathField.tsx"

export const { useAppForm } = createFormHook({
  fieldContext,
  formContext,
  fieldComponents: {
    FileOrPathField,
  },
  formComponents: {},
})
