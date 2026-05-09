declare module "livemark:virtual" {
  export const config: {
    site?: string
    title?: string
    description?: string
    logo?: string
    favicon?: string
    [key: string]: unknown
  }
}
