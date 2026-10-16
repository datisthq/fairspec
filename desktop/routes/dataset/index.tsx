import { Trans, useLingui } from "@lingui/react/macro"
import { createFileRoute, Link } from "@tanstack/react-router"
import { Card, CardDescription, CardHeader, CardTitle } from "#elements/card.tsx"
import * as icons from "#icons.ts"

export const Route = createFileRoute("/dataset/")({
  component: Component,
})

function Component() {
  const { t } = useLingui()

  const items = [
    {
      id: "validate",
      title: t`Validate Dataset`,
      icon: icons.Validate,
      path: "/dataset/validate",
      description: t`Validate dataset metadata against specifications and automatically infer dataset structure from your data files`,
      color: "text-blue-500",
    },
    {
      id: "infer",
      title: t`Infer Dataset`,
      icon: icons.Infer,
      path: "/dataset/infer",
      description: t`Automatically infer dataset metadata, table resources, and the overall structure from your raw data files`,
      color: "text-green-500",
    },
  ]

  return (
    <div className="flex flex-col gap-4">
      <div className="text-content">
        <h1 id="top">
          <Trans>Dataset</Trans>
        </h1>
        <p>
          <Trans>
            Validate dataset metadata and infer dataset descriptors from your data files
          </Trans>
          .
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
