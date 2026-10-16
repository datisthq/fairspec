export type SampleSize =
  | { type: "Records"; count: number }
  | { type: "Bytes"; count: number }
  | { type: "All" }
