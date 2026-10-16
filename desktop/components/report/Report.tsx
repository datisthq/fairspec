import type * as fairspec from "@fairspec/metadata"
import { useLingui } from "@lingui/react/macro"
import { groupBy } from "es-toolkit"
import { useState } from "react"
import { objectKeys } from "ts-extras"
import { Card, CardContent } from "#elements/card.tsx"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "#elements/tabs.tsx"
import { Error } from "./Error.tsx"

export function Report(props: { report: fairspec.Report }) {
  const { t } = useLingui()
  const { report } = props

  const errorsByType = {
    all: report.errors ?? [],
    ...groupBy(report.errors ?? [], error => error.type),
  }

  const errorTypes = objectKeys(errorsByType)
  const [selectedType, setSelectedType] = useState<string>(errorTypes?.[0] ?? "all")

  if (!report.errors?.length) {
    return null
  }

  return (
    <Tabs value={selectedType} onValueChange={value => setSelectedType(value)}>
      <div className="flex flex-col gap-8">
        <div className="flex justify-center">
          <TabsList variant="line">
            {errorTypes.map(type => {
              return (
                <TabsTrigger
                  key={type}
                  value={type}
                  className={`w-full sm:w-auto uppercase ${selectedType === type ? "font-bold" : "font-normal"}`}
                >
                  {type === "all" ? t`All Errors` : type} ({errorsByType[type].length})
                </TabsTrigger>
              )
            })}
          </TabsList>
        </div>

        {errorTypes.map(type => {
          return (
            <TabsContent key={type} value={type}>
              <div className="overflow-auto">
                <div className="flex flex-col gap-4">
                  {errorsByType[type].map((error, index) => (
                    <Card
                      key={index}
                      className="bg-red-50 dark:bg-red-950 border border-red-500"
                    >
                      <CardContent className="text-lg">
                        <Error error={error} />
                      </CardContent>
                    </Card>
                  ))}
                </div>
              </div>
            </TabsContent>
          )
        })}
      </div>
    </Tabs>
  )
}
