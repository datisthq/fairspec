use std::sync::LazyLock;

use polars::prelude::Engine;

pub const NUMBER_COLUMN_NAME: &str = "fairspec:number";
pub const ERROR_COLUMN_NAME: &str = "fairspec:error";

pub const QUERY_ENGINE: Engine = Engine::Streaming;

pub const BASE64_REGEX: &str =
    r"^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$";
pub const HEX_REGEX: &str = r"^(?:[0-9A-Fa-f]{2})*$";

// Every class is spelled out in ASCII: `\d` and `(?i)` match non-ASCII in Polars' engine
const HEX_GROUP: &str = "[0-9A-Fa-f]{1,4}";
const DEC_OCTET: &str = "(?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])";
const URL_CHARS: &str = r"A-Za-z0-9\-._~!$&'()*+,;=";
const PCT_ENCODED: &str = "%[0-9A-Fa-f]{2}";

pub static URL_REGEX: LazyLock<String> = LazyLock::new(|| {
    let ipv4_address = format!(r"{DEC_OCTET}(?:\.{DEC_OCTET}){{3}}");
    let ls32 = format!("(?:{HEX_GROUP}:{HEX_GROUP}|{ipv4_address})");
    let ipv6_address = [
        format!("(?:{HEX_GROUP}:){{6}}{ls32}"),
        format!("::(?:{HEX_GROUP}:){{5}}{ls32}"),
        format!("(?:{HEX_GROUP})?::(?:{HEX_GROUP}:){{4}}{ls32}"),
        format!("(?:(?:{HEX_GROUP}:){{0,1}}{HEX_GROUP})?::(?:{HEX_GROUP}:){{3}}{ls32}"),
        format!("(?:(?:{HEX_GROUP}:){{0,2}}{HEX_GROUP})?::(?:{HEX_GROUP}:){{2}}{ls32}"),
        format!("(?:(?:{HEX_GROUP}:){{0,3}}{HEX_GROUP})?::{HEX_GROUP}:{ls32}"),
        format!("(?:(?:{HEX_GROUP}:){{0,4}}{HEX_GROUP})?::{ls32}"),
        format!("(?:(?:{HEX_GROUP}:){{0,5}}{HEX_GROUP})?::{HEX_GROUP}"),
        format!("(?:(?:{HEX_GROUP}:){{0,6}}{HEX_GROUP})?::"),
    ]
    .join("|");
    [
        format!(r"^[Hh][Tt][Tt][Pp][Ss]?://(?:(?:[{URL_CHARS}:]|{PCT_ENCODED})*@)?"),
        format!(r"(?:\[(?:(?:{ipv6_address})|[Vv][0-9A-Fa-f]+\.[{URL_CHARS}:]+)\]"),
        format!("|(?:[{URL_CHARS}]|{PCT_ENCODED})+)(?::[0-9]*)?"),
        format!("(?:/(?:[{URL_CHARS}:@]|{PCT_ENCODED})*)*"),
        format!(r"(?:\?(?:[{URL_CHARS}:@/?]|{PCT_ENCODED})*)?"),
        format!("(?:#(?:[{URL_CHARS}:@/?]|{PCT_ENCODED})*)?$"),
    ]
    .concat()
});

const ATEXT: &str = r"[A-Za-z0-9!#$%&'*+\-/=?^_`{|}~]";
const SUB_DOMAIN: &str = "[A-Za-z0-9]+(?:-+[A-Za-z0-9]+)*";
const SNUM: &str = "(?:25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})";

// A general address literal's tag also matches `IPv6`, so it covers the IPv6 literal
pub static EMAIL_REGEX: LazyLock<String> = LazyLock::new(|| {
    [
        format!(r#"^(?:{ATEXT}+(?:\.{ATEXT}+)*|"(?:[ !#-\[\]-~]|\\[ -~])*")@"#),
        format!(r"(?:{SUB_DOMAIN}(?:\.{SUB_DOMAIN})*"),
        format!(r"|\[(?:{SNUM}(?:\.{SNUM}){{3}}|(?:-*[A-Za-z0-9])+:[!-Z^-~]+)\])$"),
    ]
    .concat()
});

const DURATION_TIME: &str =
    "[Tt][0-9]+(?:[Hh](?:[0-9]+[Mm](?:[0-9]+[Ss])?)?|[Mm](?:[0-9]+[Ss])?|[Ss])";

pub static DURATION_REGEX: LazyLock<String> = LazyLock::new(|| {
    format!(
        "^[Pp](?:[0-9]+(?:[Dd]|[Mm](?:[0-9]+[Dd])?|[Yy](?:[0-9]+[Mm](?:[0-9]+[Dd])?)?)(?:{DURATION_TIME})?|{DURATION_TIME}|[0-9]+[Ww])$"
    )
});

