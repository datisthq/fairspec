import type { ReactNode } from "react"
import * as icons from "#icons.ts"

export type StatusType = "pending" | "success" | "error" | "fault"

export interface StatusProps {
  statusType?: StatusType
  pendingTitle: string
  successTitle: string
  errorTitle: string
}

export function Status(props: StatusProps) {
  const { statusType } = props

  const getIcon = (): ReactNode => {
    if (statusType === "pending")
      return <icons.Pending className="animate-spin text-yellow-500 w-12 h-12" />
    if (statusType === "success")
      return <icons.Success className="text-green-500 w-12 h-12" />
    if (statusType === "error") return <icons.Error className="text-red-500 w-12 h-12" />
    return null
  }

  const getTitle = (): string => {
    if (statusType === "pending") return props.pendingTitle
    if (statusType === "success") return props.successTitle
    if (statusType === "error") return props.errorTitle
    return ""
  }

  return (
    <div className="flex justify-center items-center flex-col">
      <div className="flex flex-col md:flex-row items-center gap-4">
        {getIcon()}
        <span className="text-3xl md:text-4xl font-semibold">{getTitle()}</span>
      </div>
    </div>
  )
}
