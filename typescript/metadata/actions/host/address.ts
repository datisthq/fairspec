const NON_PUBLIC_NETWORKS = [
  "0.0.0.0/8",
  "10.0.0.0/8",
  "100.64.0.0/10",
  "127.0.0.0/8",
  "169.254.0.0/16",
  "172.16.0.0/12",
  "192.0.0.0/24",
  "192.0.2.0/24",
  "192.88.99.0/24",
  "192.168.0.0/16",
  "198.18.0.0/15",
  "198.51.100.0/24",
  "203.0.113.0/24",
  "224.0.0.0/4",
  "240.0.0.0/4",
  "::/96",
  "64:ff9b:1::/48",
  "100::/64",
  "2001::/23",
  "2001:db8::/32",
  "2002::/16",
  "fc00::/7",
  "fe80::/10",
  "fec0::/10",
  "ff00::/8",
].map(parseNetwork)

const IPV4_MAPPED_NETWORK = parseNetwork("::ffff:0:0/96")
const NAT64_NETWORK = parseNetwork("64:ff9b::/96")

/**
 * Whether an IP address is publicly routable; anything unparsable is not
 */
export function getIsPublicAddress(address: string): boolean {
  const ip = parseAddress(address)
  if (!ip) return false

  if (
    ip.version === 6 &&
    (getIsInNetwork(ip, IPV4_MAPPED_NETWORK) || getIsInNetwork(ip, NAT64_NETWORK))
  ) {
    return getIsPublicAddress(formatIPv4(Number(ip.value & 0xffffffffn)))
  }

  return !NON_PUBLIC_NETWORKS.some(network => getIsInNetwork(ip, network))
}

/**
 * Throw when a host resolves to an address the public host policy refuses
 */
export function assertPublicAddress(host: string, address: string) {
  if (!getIsPublicAddress(address)) {
    throw new HostPolicyError(
      `Host ${host} resolves to non-public address ${address} ` +
        "(set FAIRSPEC_HOST_POLICY=any to allow)",
    )
  }
}

export class HostPolicyError extends Error {}

type Address = { version: 4 | 6; value: bigint }
type Network = Address & { prefix: number }

function getIsInNetwork(ip: Address, network: Network) {
  if (ip.version !== network.version) return false
  const shift = BigInt((ip.version === 4 ? 32 : 128) - network.prefix)
  return ip.value >> shift === network.value >> shift
}

function parseNetwork(text: string): Network {
  const [address = "", prefix = ""] = text.split("/")
  const ip = parseAddress(address)
  if (!ip) throw new Error(`Invalid network: ${text}`)
  return { ...ip, prefix: Number(prefix) }
}

function parseAddress(text: string): Address | undefined {
  const ipv4 = parseIPv4(text)
  if (ipv4 !== undefined) return { version: 4, value: BigInt(ipv4) }

  const ipv6 = parseIPv6(text)
  if (ipv6 !== undefined) return { version: 6, value: ipv6 }

  return undefined
}

function parseIPv4(text: string) {
  const parts = text.split(".")
  if (parts.length !== 4) return undefined

  let value = 0
  for (const part of parts) {
    if (!/^(0|[1-9]\d{0,2})$/.test(part) || Number(part) > 255) return undefined
    value = value * 256 + Number(part)
  }

  return value
}

function parseIPv6(text: string) {
  const address = text.replace(/%[^%]+$/, "")
  if (!address.includes(":")) return undefined

  const halves = address.split("::")
  if (halves.length > 2) return undefined

  const head = parseIPv6Groups(halves[0] ?? "")
  const tail = halves.length === 2 ? parseIPv6Groups(halves[1] ?? "") : []
  if (!head || !tail) return undefined

  const missing = 8 - head.length - tail.length
  if (halves.length === 2 ? missing < 1 : missing !== 0) return undefined

  const groups = [...head, ...Array<number>(missing).fill(0), ...tail]
  return groups.reduce((value, group) => (value << 16n) | BigInt(group), 0n)
}

function parseIPv6Groups(text: string) {
  if (!text) return []

  const groups: number[] = []
  const parts = text.split(":")
  for (const [index, part] of parts.entries()) {
    if (index === parts.length - 1 && part.includes(".")) {
      const ipv4 = parseIPv4(part)
      if (ipv4 === undefined) return undefined
      groups.push(Math.floor(ipv4 / 65536), ipv4 % 65536)
    } else if (/^[0-9a-f]{1,4}$/i.test(part)) {
      groups.push(Number.parseInt(part, 16))
    } else {
      return undefined
    }
  }

  return groups
}

function formatIPv4(value: number) {
  return [24, 16, 8, 0].map(shift => (value >>> shift) & 255).join(".")
}
