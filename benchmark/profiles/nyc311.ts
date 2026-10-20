import type { LocalDateTimeColumn, StringColumn, TableSchema } from "@fairspec/metadata"
import type { Random } from "../generate/random.ts"
import type { Column, Profile } from "./types.ts"

const AGENCIES = [
  ["NYPD", "New York City Police Department", 32],
  ["HPD", "Department of Housing Preservation and Development", 22],
  ["DSNY", "Department of Sanitation", 12],
  ["DOT", "Department of Transportation", 10],
  ["DEP", "Department of Environmental Protection", 8],
  ["DOB", "Department of Buildings", 4],
  ["DPR", "Department of Parks and Recreation", 4],
  ["DOHMH", "Department of Health and Mental Hygiene", 3],
  ["DHS", "Department of Homeless Services", 2],
  ["TLC", "Taxi and Limousine Commission", 1],
  ["DCWP", "Department of Consumer and Worker Protection", 1],
  ["EDC", "Economic Development Corporation", 1],
] as const

const COMPLAINTS: Record<string, readonly (readonly [string, string[]])[]> = {
  NYPD: [
    ["Noise - Residential", ["Loud Music/Party", "Banging/Pounding", "Loud Talking"]],
    [
      "Illegal Parking",
      [
        "Blocked Hydrant",
        "Double Parked Blocking Traffic",
        "Posted Parking Sign Violation",
      ],
    ],
    ["Blocked Driveway", ["No Access", "Partial Access"]],
    ["Noise - Street/Sidewalk", ["Loud Music/Party", "Loud Talking"]],
    ["Abandoned Vehicle", ["With License Plate", "No License Plate"]],
  ],
  HPD: [
    ["HEAT/HOT WATER", ["ENTIRE BUILDING", "APARTMENT ONLY"]],
    ["UNSANITARY CONDITION", ["PESTS", "MOLD", "GARBAGE"]],
    ["PLUMBING", ["WATER SUPPLY", "LEAK", "TOILET"]],
    ["PAINT/PLASTER", ["WALL", "CEILING"]],
  ],
  DSNY: [
    [
      "Missed Collection",
      ["Trash", "Recycling - Paper", "Recycling - Metal/Glass/Plastic"],
    ],
    ["Dirty Condition", ["Trash", "Litter"]],
    ["Derelict Vehicles", ["Derelict Vehicles"]],
  ],
  DOT: [
    ["Street Condition", ["Pothole", "Cave-in", "Failed Street Repair"]],
    ["Street Light Condition", ["Street Light Out", "Street Light Cycling"]],
    ["Traffic Signal Condition", ["Controller", "Ped Flasher"]],
  ],
  DEP: [
    ["Water System", ["Hydrant Running", "No Water (WNW)", "Leak (Use Comments) (WA2)"]],
    [
      "Sewer",
      [
        "Catch Basin Clogged/Flooding (Use Comments) (SC)",
        "Sewer Backup (Use Comments) (SA)",
      ],
    ],
    [
      "Noise",
      [
        "Noise: Construction Before/After Hours (NM1)",
        "Noise: air condition/ventilation equipment (NV1)",
      ],
    ],
  ],
}
const COMPLAINTS_DEFAULT = [
  ["General Complaint", ["Other"]],
  ["Request Large Bulky Item Collection", ["Request Large Bulky Item Collection"]],
] as const

const BOROUGHS = [
  [
    "BROOKLYN",
    "BROOKLYN",
    31,
    ["11201", "11206", "11207", "11211", "11215", "11226", "11234"],
  ],
  ["QUEENS", "JAMAICA", 23, ["11354", "11368", "11373", "11385", "11432", "11434"]],
  ["MANHATTAN", "NEW YORK", 21, ["10002", "10009", "10025", "10027", "10031", "10032"]],
  ["BRONX", "BRONX", 20, ["10453", "10456", "10457", "10458", "10467", "10468"]],
  ["STATEN ISLAND", "STATEN ISLAND", 4, ["10301", "10304", "10305", "10306", "10314"]],
  ["Unspecified", "", 1, [""]],
] as const

const STREETS = [
  "BROADWAY",
  "AMSTERDAM AVENUE",
  "GRAND CONCOURSE",
  "OCEAN AVENUE",
  "FLATBUSH AVENUE",
  "JAMAICA AVENUE",
  "QUEENS BOULEVARD",
  "EASTERN PARKWAY",
  "LEXINGTON AVENUE",
  "VICTORY BOULEVARD",
  "WEST 181 STREET",
  "EAST 14 STREET",
  "NOSTRAND AVENUE",
  "ROOSEVELT AVENUE",
  "FORDHAM ROAD",
]
const LOCATION_TYPES = [
  ["RESIDENTIAL BUILDING", 40],
  ["Street/Sidewalk", 30],
  ["Residential Building/House", 10],
  ["Store/Commercial", 5],
  ["Park", 3],
  ["", 12],
] as const
const STATUSES = [
  ["Closed", 90],
  ["Open", 4],
  ["In Progress", 3],
  ["Pending", 1],
  ["Assigned", 1],
  ["Started", 1],
] as const
const CHANNELS = [
  ["PHONE", 45],
  ["ONLINE", 30],
  ["MOBILE", 20],
  ["UNKNOWN", 5],
] as const
const ADDRESS_TYPES = [
  ["ADDRESS", 70],
  ["INTERSECTION", 12],
  ["BLOCKFACE", 8],
  ["LATLONG", 5],
  ["", 5],
] as const
const RESOLUTIONS = [
  "The Police Department responded to the complaint and with the information available observed no evidence of the violation at that time.",
  "The Police Department responded and upon arrival those responsible for the condition were gone.",
  "The Department of Housing Preservation and Development inspected the following conditions. Violations were issued. Information about specific violations is available at www.nyc.gov/hpd.",
  "The Department of Sanitation investigated this complaint and found no violation at the location.",
  "The Department of Transportation inspected this condition and repaired the problem.",
  "Your request can not be processed at this time because of insufficient contact information.",
]

const EPOCH = Date.UTC(2010, 0, 1) / 1000
const SPAN = Date.UTC(2026, 0, 1) / 1000 - EPOCH

const string: StringColumn["property"] = { type: "string" }
const nullableString: StringColumn["property"] = { type: ["string", "null"] }
const dateTime: LocalDateTimeColumn["property"] = {
  type: "string",
  format: "local-date-time",
}
const nullableDateTime: LocalDateTimeColumn["property"] = {
  type: ["string", "null"],
  format: "local-date-time",
}

const tableSchema: TableSchema = {
  properties: {
    unique_key: { type: "integer", minimum: 1 },
    created_date: dateTime,
    closed_date: nullableDateTime,
    agency: { type: "string", enum: AGENCIES.map(([code]) => code) },
    agency_name: string,
    complaint_type: string,
    descriptor: nullableString,
    location_type: nullableString,
    incident_zip: { type: ["string", "null"], pattern: "^[0-9]{5}$" },
    incident_address: nullableString,
    street_name: nullableString,
    cross_street_1: nullableString,
    cross_street_2: nullableString,
    intersection_street_1: nullableString,
    intersection_street_2: nullableString,
    address_type: nullableString,
    city: nullableString,
    landmark: nullableString,
    facility_type: nullableString,
    status: { type: "string", enum: STATUSES.map(([status]) => status) },
    due_date: nullableDateTime,
    resolution_description: nullableString,
    resolution_action_updated_date: nullableDateTime,
    community_board: nullableString,
    bbl: { type: ["integer", "null"], minimum: 1000000000, maximum: 5999999999 },
    borough: { type: "string", enum: BOROUGHS.map(([borough]) => borough) },
    x_coordinate_state_plane: {
      type: ["integer", "null"],
      minimum: 900000,
      maximum: 1100000,
    },
    y_coordinate_state_plane: {
      type: ["integer", "null"],
      minimum: 100000,
      maximum: 300000,
    },
    open_data_channel_type: {
      type: "string",
      enum: CHANNELS.map(([channel]) => channel),
    },
    park_facility_name: nullableString,
    park_borough: nullableString,
    vehicle_type: nullableString,
    taxi_company_borough: nullableString,
    taxi_pick_up_location: nullableString,
    bridge_highway_name: nullableString,
    bridge_highway_direction: nullableString,
    road_ramp: nullableString,
    bridge_highway_segment: nullableString,
    latitude: { type: ["number", "null"], minimum: 40.49, maximum: 40.92 },
    longitude: { type: ["number", "null"], minimum: -74.26, maximum: -73.69 },
    location: nullableString,
  },
  required: [
    "unique_key",
    "created_date",
    "agency",
    "agency_name",
    "complaint_type",
    "status",
    "borough",
    "open_data_channel_type",
  ],
  primaryKey: ["unique_key"],
}

/** Draws one coherent request, so related columns agree (agency and complaint, borough and zip). */
function drawRequest(random: Random, row: number) {
  const [agency, agencyName] = random.weighted(
    AGENCIES.map(item => [item, item[2]] as const),
  )
  const [complaintType, descriptors] = random.pick(
    COMPLAINTS[agency] ?? COMPLAINTS_DEFAULT,
  )
  const [borough, city, , zips] = random.weighted(
    BOROUGHS.map(item => [item, item[2]] as const),
  )
  const created = EPOCH + random.integer(0, SPAN)
  const status = random.weighted(STATUSES)
  const closed =
    status === "Closed" ? created + random.integer(600, 30 * 86400) : undefined
  const located = borough !== "Unspecified" && random.chance(0.97)
  const latitude = 40.5 + random.float() * 0.4
  const longitude = -74.25 + random.float() * 0.55
  const street = random.pick(STREETS)
  const isVehicle = agency === "TLC"

  const request: Record<string, string> = {
    unique_key: String(10000000 + row),
    created_date: formatDateTime(created),
    closed_date: closed ? formatDateTime(closed) : "",
    agency,
    agency_name: agencyName,
    complaint_type: complaintType,
    descriptor: random.pick(descriptors),
    location_type: random.weighted(LOCATION_TYPES),
    incident_zip: random.pick(zips),
    incident_address: located ? `${random.integer(1, 3999)} ${street}` : "",
    street_name: located ? street : "",
    cross_street_1: random.chance(0.6) ? random.pick(STREETS) : "",
    cross_street_2: random.chance(0.6) ? random.pick(STREETS) : "",
    intersection_street_1: random.chance(0.2) ? random.pick(STREETS) : "",
    intersection_street_2: random.chance(0.2) ? random.pick(STREETS) : "",
    address_type: random.weighted(ADDRESS_TYPES),
    city,
    landmark: random.chance(0.05) ? street : "",
    facility_type: random.chance(0.1) ? "N/A" : "",
    status,
    due_date: random.chance(0.15) ? formatDateTime(created + 8 * 86400) : "",
    resolution_description: closed ? random.pick(RESOLUTIONS) : "",
    resolution_action_updated_date: closed ? formatDateTime(closed) : "",
    community_board:
      borough === "Unspecified"
        ? "0 Unspecified"
        : `${String(random.integer(1, 18)).padStart(2, "0")} ${borough}`,
    bbl:
      located && random.chance(0.8) ? String(random.integer(1000000000, 5999999999)) : "",
    borough,
    x_coordinate_state_plane: located ? String(random.integer(913000, 1067000)) : "",
    y_coordinate_state_plane: located ? String(random.integer(121000, 271000)) : "",
    open_data_channel_type: random.weighted(CHANNELS),
    park_facility_name: agency === "DPR" ? "Central Park" : "Unspecified",
    park_borough: borough,
    vehicle_type: isVehicle ? "Car Service" : "",
    taxi_company_borough: isVehicle ? borough : "",
    taxi_pick_up_location: isVehicle ? "Other" : "",
    bridge_highway_name: random.chance(0.01) ? "Brooklyn Bridge" : "",
    bridge_highway_direction: random.chance(0.01) ? "Manhattan Bound" : "",
    road_ramp: random.chance(0.01) ? "Roadway" : "",
    bridge_highway_segment: random.chance(0.01) ? "Brooklyn Bridge Roadway" : "",
    latitude: located ? latitude.toFixed(15) : "",
    longitude: located ? longitude.toFixed(15) : "",
    location: located ? `(${latitude.toFixed(15)}, ${longitude.toFixed(15)})` : "",
  }
  return request
}

const NAMES = Object.keys(tableSchema.properties ?? {})

/** Columns share one drawn request per row; `unique_key` is first, so it starts each row. */
function createColumns(): Column[] {
  let current: Record<string, string> = {}
  return NAMES.map((name, index) => ({
    name,
    generate: (random, row) => {
      if (index === 0) current = drawRequest(random, row)
      return current[name] ?? ""
    },
  }))
}

function formatDateTime(seconds: number) {
  return new Date(seconds * 1000).toISOString().slice(0, 19)
}

export const nyc311Profile: Profile = {
  name: "nyc-311",
  description: "The 41 columns of NYC 311 Service Requests, with plausible value spreads",
  tableSchema,
  columns: createColumns(),
  faults: [
    { column: "x_coordinate_state_plane", check: "type", value: "east" },
    { column: "agency", check: "required", value: "" },
    { column: "latitude", check: "range", value: "41.500000000000000" },
    { column: "borough", check: "enum", value: "GOTHAM" },
    { column: "incident_zip", check: "pattern", value: "1O001" },
  ],
}
