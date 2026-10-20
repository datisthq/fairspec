import type { Profile } from "./types.ts"

const CATEGORIES = ["alpha", "beta", "gamma", "delta", "epsilon"]
const COUNTRIES = ["US", "GB", "DE", "FR", "IT", "ES", "JP", "BR", "IN", "CA"]
const WORDS = [
  "lorem",
  "ipsum",
  "dolor",
  "sit",
  "amet",
  "consectetur",
  "adipiscing",
  "elit",
]

export const simpleProfile: Profile = {
  name: "simple",
  description: "Ten columns of mixed types, one of each common constraint",
  tableSchema: {
    properties: {
      id: { type: "integer", minimum: 1 },
      name: { type: "string", minLength: 1 },
      email: { type: "string", pattern: "^[a-z0-9.]+@[a-z0-9.]+$" },
      age: { type: ["integer", "null"], minimum: 0, maximum: 120 },
      score: { type: "number", minimum: 0, maximum: 100 },
      active: { type: "boolean" },
      category: { type: "string", enum: CATEGORIES },
      created: { type: "string", format: "date-time" },
      country: { type: ["string", "null"], pattern: "^[A-Z]{2}$" },
      notes: { type: ["string", "null"] },
    },
    required: ["id", "name", "email", "score", "active", "category", "created"],
    primaryKey: ["id"],
  },
  columns: [
    { name: "id", generate: (_, row) => String(row) },
    { name: "name", generate: random => `${random.pick(WORDS)} ${random.pick(WORDS)}` },
    {
      name: "email",
      generate: (random, row) => `${random.pick(WORDS)}.${row}@example.org`,
    },
    { name: "age", generate: random => String(random.integer(0, 120)) },
    { name: "score", generate: random => (random.float() * 100).toFixed(2) },
    { name: "active", generate: random => String(random.chance(0.5)) },
    { name: "category", generate: random => random.pick(CATEGORIES) },
    {
      name: "created",
      generate: random =>
        new Date(1577836800000 + random.integer(0, 157680000) * 1000)
          .toISOString()
          .slice(0, 19),
    },
    { name: "country", generate: random => random.pick(COUNTRIES) },
    {
      name: "notes",
      generate: random =>
        random.chance(0.7) ? "" : `${random.pick(WORDS)} ${random.pick(WORDS)}`,
    },
  ],
  faults: [
    { column: "age", check: "type", value: "forty" },
    { column: "name", check: "required", value: "" },
    { column: "score", check: "range", value: "150.5" },
    { column: "category", check: "enum", value: "omega" },
    { column: "country", check: "pattern", value: "usa" },
  ],
}
