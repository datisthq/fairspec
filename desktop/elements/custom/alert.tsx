import { cva, type VariantProps } from "class-variance-authority"
import { cn } from "cn"
import * as icons from "#icons.ts"

const alertVariants = cva(
  "grid gap-0.5 rounded-lg border px-4 py-3 text-left text-sm has-data-[slot=alert-action]:relative has-data-[slot=alert-action]:pr-18 has-[>svg]:grid-cols-[auto_1fr] has-[>svg]:gap-x-2.5 *:[svg]:row-span-2 *:[svg]:translate-y-0.5 *:[svg]:text-current *:[svg:not([class*='size-'])]:size-4 w-full relative group/alert",
  {
    variants: {
      variant: {
        default: "bg-card text-card-foreground",
        danger:
          "text-red-600 dark:text-white bg-red-50 dark:bg-red-800 border-red-300 dark:border-red-700 *:data-[slot=alert-description]:text-red-600 dark:*:data-[slot=alert-description]:text-white/90 *:[svg]:text-current",
        warning:
          "text-amber-700 dark:text-white bg-amber-100 dark:bg-amber-900 border-amber-300 dark:border-amber-800 *:data-[slot=alert-description]:text-amber-700 dark:*:data-[slot=alert-description]:text-white/90 *:[svg]:text-current",
        success:
          "text-green-700 dark:text-white bg-green-100 dark:bg-green-900 border-green-300 dark:border-green-800 *:data-[slot=alert-description]:text-green-700 dark:*:data-[slot=alert-description]:text-white/90 *:[svg]:text-current",
        note: "text-blue-700 dark:text-white bg-blue-100 dark:bg-blue-900 border-blue-300 dark:border-blue-800 *:data-[slot=alert-description]:text-blue-700 dark:*:data-[slot=alert-description]:text-white/90 *:[svg]:text-current",
        tip: "bg-purple-100 dark:bg-purple-900 text-purple-900 dark:text-white border-purple-300 dark:border-purple-800 *:data-[slot=alert-description]:text-purple-800 dark:*:data-[slot=alert-description]:text-white/90 *:[svg]:text-current",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  },
)

const variantIcons = {
  danger: icons.Danger,
  warning: icons.Warning,
  success: icons.Success,
  note: icons.Note,
  tip: icons.Tip,
} as const

function Alert({
  className,
  variant,
  icon,
  children,
  ...props
}: React.ComponentProps<"div"> &
  VariantProps<typeof alertVariants> & {
    icon?: React.ComponentType<{ className?: string }> | null
  }) {
  const DefaultIcon =
    variant && variant in variantIcons
      ? variantIcons[variant as keyof typeof variantIcons]
      : null

  const Icon = icon === undefined ? DefaultIcon : icon

  return (
    <div
      data-slot="alert"
      role="alert"
      className={cn(alertVariants({ variant }), className)}
      {...props}
    >
      {Icon && <Icon className="size-6" />}
      {children}
    </div>
  )
}

function AlertTitle({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="alert-title"
      className={cn(
        "font-medium group-has-[>svg]/alert:col-start-2 [&_a]:underline [&_a]:underline-offset-3 [&_a]:hover:opacity-80",
        className,
      )}
      {...props}
    />
  )
}

function AlertDescription({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="alert-description"
      className={cn(
        "text-muted-foreground text-sm text-balance md:text-pretty [&_p:not(:last-child)]:mb-4 [&_a]:underline [&_a]:underline-offset-3 [&_a]:hover:opacity-80",
        className,
      )}
      {...props}
    />
  )
}

function AlertAction({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="alert-action"
      className={cn("absolute top-2.5 right-3", className)}
      {...props}
    />
  )
}

export { Alert, AlertTitle, AlertDescription, AlertAction }
