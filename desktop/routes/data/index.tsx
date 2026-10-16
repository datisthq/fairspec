import { Trans, useLingui } from "@lingui/react/macro"
import { createFileRoute, Link } from "@tanstack/react-router"
import { Card, CardDescription, CardHeader, CardTitle } from "#elements/card.tsx"
import * as icons from "#icons.ts"

export const Route = createFileRoute("/data/")({
  component: Component,
})

function Component() {
  const { t } = useLingui()

  const items = [
    {
      id: "validate",
      title: t`Validate Data`,
      icon: icons.Validate,
      path: "/data/validate",
      description: t`Check data values against schema specifications for quality, consistency, and constraint violations`,
      color: "text-blue-500",
    },
    {
      id: "infer-schema",
      title: t`Infer Schema`,
      icon: icons.Infer,
      path: "/data/infer-schema",
      description: t`Automatically derive a comprehensive JSON Schema definition from your data's structure and values`,
      color: "text-green-500",
    },
    {
      id: "infer-dialect",
      title: t`Infer Dialect`,
      icon: icons.Infer,
      path: "/file/infer-dialect",
      description: t`Detect file formats, encoding specifications, delimiters, and other dialect parameters automatically`,
      color: "text-purple-500",
    },
  ]

  return (
    <div className="flex flex-col gap-4">
      <div className="text-content">
        <h1 id="top">
          <Trans>Data</Trans>
        </h1>
        <p>
          <Trans>Validate data values and infer JSON Schemas from your datasets</Trans>.
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
