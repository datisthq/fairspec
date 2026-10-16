export function getFileBasename(file: File | string) {
  const filename = typeof file === "string" ? file : file.name
  const name = filename.split(/[\\/]/).pop() || filename
  const dotIndex = name.lastIndexOf(".")
  return dotIndex > 0 ? name.slice(0, dotIndex) : name
}
