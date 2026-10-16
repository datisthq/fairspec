import { Trans, useLingui } from "@lingui/react/macro"
import { createFileRoute, Link } from "@tanstack/react-router"
import { Card, CardDescription, CardHeader, CardTitle } from "#elements/card.tsx"
import * as icons from "#icons.ts"

export const Route = createFileRoute("/table/")({
  component: Component,
})

function Component() {
  const { t } = useLingui()

  const items = [
    {
      id: "preview",
      title: t`Preview Table`,
      icon: icons.Preview,
      path: "/table/preview",
      description: t`Load tabular data from your files and preview rows, columns, and overall structure to inspect contents`,
      color: "text-blue-500",
    },
    {
      id: "validate",
      title: t`Validate Table`,
      icon: icons.Validate,
      path: "/table/validate",
      description: t`Validate table structure for correctness and compliance, and automatically infer table schema from your tabular data`,
      color: "text-green-500",
    },
    {
      id: "infer-schema",
      title: t`Infer Schema`,
      icon: icons.Infer,
      path: "/table/infer-schema",
      description: t`Automatically infer comprehensive table schema definitions with column types and constraints from your tabular data`,
      color: "text-purple-500",
    },
    {
      id: "infer-dialect",
      title: t`Infer Dialect`,
      icon: icons.Infer,
      path: "/file/infer-dialect",
      description: t`Detect file formats, encoding specifications, delimiters, and other dialect parameters automatically`,
      color: "text-orange-500",
    },
  ]

  return (
    <div className="flex flex-col gap-4">
      <div className="text-content">
        <h1 id="top">
          <Trans>Table</Trans>
        </h1>
        <p>
          <Trans>Preview, validate, and infer schemas for tabular data</Trans>.
        </p>
      </div>
      <div className="py-4">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          {items.map(item => {
            const Icon = item.icon
            return (
              <Link key={item.id} to={item.path}>
                <Card className="hover:shadow-lg transition-shadow cursor-pointer h-full group">
                  <CardHeader>
                    <div className="space-y-3">
                      <div className="flex items-center gap-3">
                        <div className={item.color}>
                          <Icon className="w-6 h-6 group-hover:animate-[spin_0.5s_ease-in-out_1]" />
                        </div>
                        <CardTitle className="text-xl">{item.title}</CardTitle>
                      </div>
                      <CardDescription className="text-base">
                        {item.description}
                      </CardDescription>
                    </div>
                  </CardHeader>
                </Card>
              </Link>
            )
          })}
        </div>
      </div>
    </div>
  )
}
