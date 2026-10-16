export function Code(props: React.ComponentProps<"code">) {
  return (
    <code className="text-lg font-bold bg-white dark:bg-black px-1 py-0.5 rounded">
      {props.children}
    </code>
  )
}
