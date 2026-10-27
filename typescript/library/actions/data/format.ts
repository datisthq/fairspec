import { DURATION_REGEX, EMAIL_REGEX } from "@fairspec/table"

const DATE_REGEX = /^([0-9]{4})-([0-9]{2})-([0-9]{2})$/
const TIME_REGEX =
  /^([0-9]{2}):([0-9]{2}):([0-9]{2})(?:\.[0-9]+)?(?:[Zz]|([+-])([0-9]{2}):([0-9]{2}))$/
const DEC_OCTET = "(?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])"
const IPV4_REGEX = new RegExp(`^${DEC_OCTET}(?:\\.${DEC_OCTET}){3}$`)
const UUID_REGEX =
  /^[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}$/
const EMAIL_PATTERN = new RegExp(EMAIL_REGEX)
const DURATION_PATTERN = new RegExp(DURATION_REGEX)

/**
 * The `format` values a Data Schema asserts; any other format is an annotation
 */
export const DATA_SCHEMA_FORMATS: Record<string, (value: string) => boolean> = {
  date: getIsDate,
  time: getIsTime,
  "date-time": value =>
    /^[Tt]$/.test(value.charAt(10)) &&
    getIsDate(value.slice(0, 10)) &&
    getIsTime(value.slice(11)),
  duration: value => DURATION_PATTERN.test(value),
  email: value => EMAIL_PATTERN.test(value),
  ipv4: value => IPV4_REGEX.test(value),
  uuid: value => UUID_REGEX.test(value),
}

function getIsDate(value: string) {
  const match = DATE_REGEX.exec(value)
  if (!match) return false

  const year = Number(match[1])
  const month = Number(match[2])
  const day = Number(match[3])
  const isLeap = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0)
  const days = [31, isLeap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
  return month >= 1 && month <= 12 && day >= 1 && day <= (days[month - 1] ?? 0)
}

// RFC 3339 allows a leap second only at the last minute of a UTC day
function getIsTime(value: string) {
  const match = TIME_REGEX.exec(value)
  if (!match) return false

  const hour = Number(match[1])
  const minute = Number(match[2])
  const second = Number(match[3])
  const sign = match[4] === "-" ? -1 : 1
  const offsetHour = Number(match[5] ?? 0)
  const offsetMinute = Number(match[6] ?? 0)
  if (hour > 23 || minute > 59 || second > 60) return false
  if (offsetHour > 23 || offsetMinute > 59) return false
  if (second < 60) return true

  const utcMinutes = hour * 60 + minute - sign * (offsetHour * 60 + offsetMinute)
  return ((utcMinutes % 1440) + 1440) % 1440 === 23 * 60 + 59
}
