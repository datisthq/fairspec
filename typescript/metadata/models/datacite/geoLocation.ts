import { z } from "zod"
import { Latitude, Longitude } from "./common.ts"

export const GeoLocationPoint = z.object({
  pointLongitude: Longitude.describe("Longitudinal dimension of point"),
  pointLatitude: Latitude.describe("Latitudinal dimension of point"),
})

export const GeoLocationBox = z.object({
  westBoundLongitude: Longitude.describe("Western longitudinal dimension of box"),
  eastBoundLongitude: Longitude.describe("Eastern longitudinal dimension of box"),
  southBoundLatitude: Latitude.describe("Southern latitudinal dimension of box"),
  northBoundLatitude: Latitude.describe("Northern latitudinal dimension of box"),
})

export const GeoLocationPolygonItem = z.object({
  polygonPoint: GeoLocationPoint.optional().describe("A point location in a polygon"),
  inPolygonPoint: GeoLocationPoint.optional().describe(
    "For any bound area that is larger than half the earth, define a (random) point inside",
  ),
})

export const GeoLocation = z.object({
  geoLocationPlace: z
    .string()
    .optional()
    .describe(
      "Spatial region or named place where the data was gathered or about which the data is focused",
    ),
  geoLocationPoint: GeoLocationPoint.optional().describe("A point location in space"),
  geoLocationBox: GeoLocationBox.optional().describe("The spatial limits of a box"),
  geoLocationPolygon: z
    .array(GeoLocationPolygonItem)
    .optional()
    .describe(
      "A drawn polygon area, defined by a set of points and lines connecting the points in a closed chain",
    ),
})

export const GeoLocations = z
  .array(GeoLocation)
  .describe(
    "Spatial region or named place where the data was gathered or about which the data is focused",
  )

export type GeoLocationPoint = z.infer<typeof GeoLocationPoint>
export type GeoLocationBox = z.infer<typeof GeoLocationBox>
export type GeoLocationPolygonItem = z.infer<typeof GeoLocationPolygonItem>
export type GeoLocation = z.infer<typeof GeoLocation>
export type GeoLocations = z.infer<typeof GeoLocations>
