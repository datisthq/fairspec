pub const NUMBER_COLUMN_NAME: &str = "fairspec:number";
pub const ERROR_COLUMN_NAME: &str = "fairspec:error";

pub const BASE64_REGEX: &str =
    r"^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$";
pub const HEX_REGEX: &str = r"^[0-9a-fA-F]*$";
pub const RFC5322_EMAIL_REGEX: &str = r#"^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$"#;
pub const URL_REGEX: &str = r"^https?://.+";
