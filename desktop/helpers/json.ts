export async function saveJson(value: unknown, fileName: string) {
  const content = JSON.stringify(value, null, 2)

  const filePath = await globalThis.desktop.saveFileDialog({
    defaultPath: fileName,
    filters: [
      { name: "JSON Files", extensions: ["json"] },
      { name: "All Files", extensions: ["*"] },
    ],
  })

  if (!filePath) {
    return
  }

  await globalThis.desktop.writeFile({ filePath, content })
  return filePath
}
