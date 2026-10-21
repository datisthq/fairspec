use serde::{Deserialize, Serialize};

pub type Subjects = Vec<Subject>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub subject: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}
