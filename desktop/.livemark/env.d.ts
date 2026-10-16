declare module "livemark:virtual" {
  export const config: {
    site?: string
    title?: string
    description?: string
    [key: string]: unknown
  }
}
