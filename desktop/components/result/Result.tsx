import { Trans } from "@lingui/react/macro"
import type { ReactNode } from "react"
import { Button } from "#elements/button.tsx"
import {
  Drawer,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
} from "#elements/drawer.tsx"

export function Result(props: {
  open?: boolean
  status: ReactNode
  children?: ReactNode
  action?: ReactNode
  onOpenChange: (open: boolean) => void
}) {
  return (
    <Drawer open={props.open} onOpenChange={props.onOpenChange}>
      <DrawerContent>
        <DrawerHeader>
          <DrawerTitle className="mt-4">{props.status}</DrawerTitle>
        </DrawerHeader>
        {props.children && (
          <div className="my-4 max-w-6xl w-full mx-auto px-4 flex-1 overflow-y-auto min-h-0">
            {props.children}
          </div>
        )}
        <DrawerFooter className="max-w-6xl w-full mx-auto flex flex-col gap-4">
          {props.action}
          <Button
            onClick={() => props.onOpenChange(false)}
            variant="outline"
            className="w-full text-xl h-12"
            size="lg"
          >
            <Trans>Close</Trans>
          </Button>
        </DrawerFooter>
      </DrawerContent>
    </Drawer>
  )
}
