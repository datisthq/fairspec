import { useLingui } from "@lingui/react/macro"
import { useLocation } from "@tanstack/react-router"
import type { FileType } from "#models/file.ts"

export interface HelpField {
  label: string
  description: string
  fileType: FileType
  placeholder?: string
  required?: boolean
}

export interface HelpEntry {
  summary: string
  steps?: string[]
  result?: string
  fields: Record<string, HelpField>
  link: { url: string; label: string }
}

/**
 * Contextual help for the current route, used by the right rail and by the
 * routes themselves for their form field props.
 *
 * This is a hook rather than a module constant on purpose: Lingui macros
 * evaluated at module scope resolve once at import time, before the locale is
 * activated, so the copy would be stuck in English and would not follow a
 * language change.
 */
export function useHelp(): HelpEntry | undefined {
  const { t } = useLingui()
  const pathname = useLocation({ select: location => normalizePath(location.pathname) })

  const docs = t`Read more`

  const upload = {
    dataset: t`Choose a file or provide a URL to a dataset`,
    table: t`Choose a file or provide a URL to a tabular data file`,
    tableSchema: t`Choose a file or provide a URL to a table schema`,
    tableDialect: t`Choose a file or provide a URL to a table dialect`,
    data: t`Choose a file or provide a URL to a data file`,
    dataSchema: t`Choose a file or provide a URL to a data schema`,
    file: t`Choose a file or provide a URL to a file`,
  }

  const entries: Record<string, HelpEntry> = {
    "/": {
      summary: t`Fairspec describes datasets, tabular data and files with plain JSON descriptors, so that data can be published with enough structure to be checked. This application validates those descriptors and the data they point at, and infers them from data you already have, all on your own machine`,
      fields: {},
      link: { url: "https://fairspec.org/overview/", label: docs },
    },

    "/dataset": {
      summary: t`Validate dataset metadata and infer dataset descriptors from your data files`,
      fields: {},
      link: { url: "https://fairspec.org/specs/dataset/", label: docs },
    },
    "/dataset/validate": {
      summary: t`Checks a dataset descriptor against the Fairspec specification and confirms that every resource it references is consistent`,
      steps: [
        t`Point at a dataset descriptor, usually a dataset.json sitting beside your data files`,
        t`Every resource the descriptor references is resolved and checked, so keep it next to the data it describes`,
        t`Fix the reported paths or metadata and validate again`,
      ],
      result: t`A report locating each violation inside the descriptor`,
      fields: {
        dataset: {
          label: t`Dataset`,
          description: upload.dataset,
          placeholder: "https://example.com/dataset.json",
          fileType: "dataset",
          required: true,
        },
      },
      link: { url: "https://fairspec.org/specs/dataset/", label: docs },
    },
    "/dataset/infer": {
      summary: t`Builds a dataset descriptor from a tabular file, inferring the resource, its schema and its dialect`,
      steps: [
        t`Point at one tabular file; the descriptor is built around it as a single resource`,
        t`Review the generated metadata and add a title, licence and description by hand`,
        t`Save it as dataset.json beside your data, then check it with Validate Dataset`,
      ],
      result: t`A dataset descriptor you can save and edit`,
      fields: {
        table: {
          label: t`Table`,
          description: upload.table,
          placeholder: "https://example.com/file.csv",
          fileType: "table",
          required: true,
        },
      },
      link: { url: "https://fairspec.org/specs/dataset/", label: docs },
    },

    "/table": {
      summary: t`Preview, validate, and infer schemas for tabular data`,
      fields: {},
      link: { url: "https://fairspec.org/specs/table-schema/", label: docs },
    },
    "/table/preview": {
      summary: t`Shows how a tabular file is parsed, with the start of the data and the table schema used to read it`,
      steps: [
        t`Use this to see how a file is actually parsed before you validate anything`,
        t`Leave the schema empty to see the one inferred from the data, or supply a schema to read the file through it`,
        t`If the columns look wrong, the dialect is being misdetected — supply one`,
      ],
      result: t`The start of the file alongside the table schema used to read it`,
      fields: {
        table: {
          label: t`Table`,
          description: upload.table,
          placeholder: "https://example.com/table.csv",
          fileType: "table",
          required: true,
        },
        schema: {
          label: t`Schema`,
          description: upload.tableSchema,
          placeholder: "https://example.com/table.schema.json",
          fileType: "schema",
        },
        dialect: {
          label: t`Dialect`,
          description: upload.tableDialect,
          placeholder: "https://example.com/table.dialect.json",
          fileType: "dialect",
        },
      },
      link: { url: "https://fairspec.org/specs/table-schema/", label: docs },
    },
    "/table/validate": {
      summary: t`Checks tabular data against a table schema, optionally with an explicit file dialect`,
      steps: [
        t`Bring a table schema to check against, or infer one first if you do not have it yet`,
        t`Failures are grouped by error type, so the problem that dominates is visible at a glance`,
        t`Supply a dialect when the delimiter or header row of the file are being misdetected`,
      ],
      result: t`A report of every cell that violates the schema`,
      fields: {
        table: {
          label: t`Table`,
          description: upload.table,
          placeholder: "https://example.com/table.csv",
          fileType: "table",
          required: true,
        },
        schema: {
          label: t`Schema`,
          description: upload.tableSchema,
          placeholder: "https://example.com/table.schema.json",
          fileType: "schema",
        },
        dialect: {
          label: t`Dialect`,
          description: upload.tableDialect,
          placeholder: "https://example.com/table.dialect.json",
          fileType: "dialect",
        },
      },
      link: { url: "https://fairspec.org/specs/table-schema/", label: docs },
    },
    "/table/infer-schema": {
      summary: t`Derives a table schema from a tabular file, with field names, types and constraints`,
      steps: [
        t`Point at a tabular file; each column is analysed to pick the narrowest type that fits`,
        t`Treat the result as a starting point rather than a finished specification`,
        t`Save it, tighten the constraints by hand, then use it with Validate Table`,
      ],
      result: t`A table schema you can save as JSON`,
      fields: {
        table: {
          label: t`Table`,
          description: upload.table,
          placeholder: "https://example.com/table.csv",
          fileType: "table",
          required: true,
        },
        dialect: {
          label: t`Dialect`,
          description: upload.tableDialect,
          placeholder: "https://example.com/table.dialect.json",
          fileType: "dialect",
        },
      },
      link: { url: "https://fairspec.org/specs/table-schema/", label: docs },
    },

    "/data": {
      summary: t`Validate data values and infer JSON Schemas from your datasets`,
      fields: {},
      link: { url: "https://fairspec.org/specs/data-schema/", label: docs },
    },
    "/data/validate": {
      summary: t`Checks structured data against a data schema`,
      steps: [
        t`Bring a JSON data file and the data schema it is supposed to satisfy`,
        t`Wrong types, violated constraints, missing required values and broken key relationships are all reported`,
        t`Infer a schema first if you do not have one yet`,
      ],
      result: t`A report of every value that violates the schema`,
      fields: {
        data: {
          label: t`Data`,
          description: upload.data,
          placeholder: "https://example.com/data.json",
          fileType: "data",
          required: true,
        },
        schema: {
          label: t`Schema`,
          description: upload.dataSchema,
          placeholder: "https://example.com/data.schema.json",
          fileType: "schema",
        },
      },
      link: { url: "https://fairspec.org/specs/data-schema/", label: docs },
    },
    "/data/infer-schema": {
      summary: t`Derives a JSON Schema from a data sample`,
      steps: [
        t`Point at a JSON data sample; its structure and values are examined`,
        t`The result describes only what the sample satisfies, so a narrow sample gives a narrow schema`,
        t`Add formats, ranges and enumerations by hand to make it stricter`,
      ],
      result: t`A JSON Schema you can save and edit`,
      fields: {
        data: {
          label: t`Data`,
          description: upload.data,
          placeholder: "https://example.com/data.json",
          fileType: "data",
          required: true,
        },
      },
      link: { url: "https://fairspec.org/specs/data-schema/", label: docs },
    },

    "/file": {
      summary: t`Validate files and infer their dialects`,
      fields: {},
      link: { url: "https://fairspec.org/specs/file-dialect/", label: docs },
    },
    "/file/infer-dialect": {
      summary: t`Detects how a tabular file is formatted by sampling its contents`,
      steps: [
        t`Point at a tabular file whose format you are unsure of`,
        t`The delimiter, quote character, header row, line terminator and encoding are read from a sample`,
        t`Save the dialect and pass it to the table tools when detection is getting it wrong`,
      ],
      result: t`A file dialect you can save as JSON`,
      fields: {
        file: {
          label: t`File`,
          description: upload.file,
          placeholder: "https://example.com/file.csv",
          fileType: "file",
          required: true,
        },
      },
      link: { url: "https://fairspec.org/specs/file-dialect/", label: docs },
    },
    "/file/validate": {
      summary: t`Verifies a file against an expected checksum`,
      steps: [
        t`Point at the file you received and paste the checksum that was published alongside it`,
        t`Pick the algorithm matching that checksum`,
        t`A match confirms the bytes are unchanged; a mismatch means the file differs from the one published`,
      ],
      result: t`A report saying whether the file matches the expected checksum`,
      fields: {
        file: {
          label: t`File`,
          description: upload.file,
          placeholder: "https://example.com/file.csv",
          fileType: "file",
          required: true,
        },
      },
      link: { url: "https://fairspec.org/specs/dataset/#integrity", label: docs },
    },
  }

  return entries[pathname]
}

/**
 * Field props for the current route, keyed by form field name. Returns a
 * placeholder rather than undefined so routes can spread the result directly.
 */
export function useHelpFields() {
  const entry = useHelp()

  return (name: string): HelpField => {
    return entry?.fields[name] ?? { label: name, description: "", fileType: "file" }
  }
}

function normalizePath(pathname: string) {
  if (pathname.length > 1 && pathname.endsWith("/")) {
    return pathname.slice(0, -1)
  }

  return pathname
}
