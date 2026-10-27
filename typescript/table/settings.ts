export const NUMBER_COLUMN_NAME = "fairspec:number"
export const ERROR_COLUMN_NAME = "fairspec:error"
export const COUNT_COLUMN_NAME = "fairspec:count"

// nodejs-polars takes an engine on collectSync only, and polars ignores the
// `streaming` option of async collect, but it reads this variable on every query.
// Without it a 1GB validation takes 24s in 8GB instead of 7s in 3GB; "0" opts out.
process.env.POLARS_FORCE_STREAMING ??= "1"

// A key probe holds every distinct key in memory (3GB for 10M rows), so they stay serial.
export const INSPECT_ROW_CONCURRENCY = 1

export const BASE64_REGEX =
  "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$"
export const HEX_REGEX = "^(?:[0-9A-Fa-f]{2})*$"

// Every class is spelled out in ASCII: `\d` and `(?i)` match non-ASCII in Polars' engine
const HEX_GROUP = "[0-9A-Fa-f]{1,4}"
const DEC_OCTET = "(?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])"
const IPV4_ADDRESS = `${DEC_OCTET}(?:\\.${DEC_OCTET}){3}`
const LS32 = `(?:${HEX_GROUP}:${HEX_GROUP}|${IPV4_ADDRESS})`
const IPV6_ADDRESS = `(?:${[
  `(?:${HEX_GROUP}:){6}${LS32}`,
  `::(?:${HEX_GROUP}:){5}${LS32}`,
  `(?:${HEX_GROUP})?::(?:${HEX_GROUP}:){4}${LS32}`,
  `(?:(?:${HEX_GROUP}:){0,1}${HEX_GROUP})?::(?:${HEX_GROUP}:){3}${LS32}`,
  `(?:(?:${HEX_GROUP}:){0,2}${HEX_GROUP})?::(?:${HEX_GROUP}:){2}${LS32}`,
  `(?:(?:${HEX_GROUP}:){0,3}${HEX_GROUP})?::${HEX_GROUP}:${LS32}`,
  `(?:(?:${HEX_GROUP}:){0,4}${HEX_GROUP})?::${LS32}`,
  `(?:(?:${HEX_GROUP}:){0,5}${HEX_GROUP})?::${HEX_GROUP}`,
  `(?:(?:${HEX_GROUP}:){0,6}${HEX_GROUP})?::`,
].join("|")})`
const URL_CHARS = "A-Za-z0-9\\-._~!$&'()*+,;="
const PCT_ENCODED = "%[0-9A-Fa-f]{2}"
export const URL_REGEX = [
  `^[Hh][Tt][Tt][Pp][Ss]?://(?:(?:[${URL_CHARS}:]|${PCT_ENCODED})*@)?`,
  `(?:\\[(?:${IPV6_ADDRESS}|[Vv][0-9A-Fa-f]+\\.[${URL_CHARS}:]+)\\]`,
  `|(?:[${URL_CHARS}]|${PCT_ENCODED})+)(?::[0-9]*)?`,
  `(?:/(?:[${URL_CHARS}:@]|${PCT_ENCODED})*)*`,
  `(?:\\?(?:[${URL_CHARS}:@/?]|${PCT_ENCODED})*)?`,
  `(?:#(?:[${URL_CHARS}:@/?]|${PCT_ENCODED})*)?$`,
].join("")

const ATEXT = "[A-Za-z0-9!#$%&'*+\\-/=?^_`{|}~]"
const SUB_DOMAIN = "[A-Za-z0-9]+(?:-+[A-Za-z0-9]+)*"
const SNUM = "(?:25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})"
// A general address literal's tag also matches `IPv6`, so it covers the IPv6 literal
export const EMAIL_REGEX = [
  `^(?:${ATEXT}+(?:\\.${ATEXT}+)*|"(?:[ !#-\\[\\]-~]|\\\\[ -~])*")@`,
  `(?:${SUB_DOMAIN}(?:\\.${SUB_DOMAIN})*`,
  `|\\[(?:${SNUM}(?:\\.${SNUM}){3}|(?:-*[A-Za-z0-9])+:[!-Z^-~]+)\\])$`,
].join("")

const DURATION_TIME =
  "[Tt][0-9]+(?:[Hh](?:[0-9]+[Mm](?:[0-9]+[Ss])?)?|[Mm](?:[0-9]+[Ss])?|[Ss])"
export const DURATION_REGEX = [
  "^[Pp](?:[0-9]+(?:[Dd]|[Mm](?:[0-9]+[Dd])?|[Yy](?:[0-9]+[Mm](?:[0-9]+[Dd])?)?)",
  `(?:${DURATION_TIME})?|${DURATION_TIME}|[0-9]+[Ww])$`,
].join("")
