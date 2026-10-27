use crate::models::descriptor::Descriptor;
use serde_json::Value;

use crate::actions::frictionless::mapping::FrictionlessRecorder;
use crate::actions::frictionless::value::get_text;

pub fn convert_rights_from_frictionless(
    licenses: &[Value],
    mut record: Option<&mut FrictionlessRecorder>,
) -> Vec<Value> {
    licenses
        .iter()
        .enumerate()
        .filter_map(|(index, license)| Some((index, license.as_object()?)))
        .map(|(index, license)| {
            let mut rights = Descriptor::new();
            let title = license
                .get("title")
                .or_else(|| license.get("name"))
                .and_then(Value::as_str)
                .filter(|title| !title.is_empty());
            if let Some(title) = title {
                rights.insert("rights".to_string(), title.into());
            }
            if let Some(path) = get_text(license, "path") {
                rights.insert("rightsUri".to_string(), path.into());
            }
            if let Some(name) = get_text(license, "name") {
                rights.insert("rightsIdentifier".to_string(), name.into());
            }

            if let Some(record) = record.as_deref_mut() {
                let title = match get_text(license, "title") {
                    Some(_) => "title",
                    None => "name",
                };
                record.record(&format!("/{index}"), &format!("/{index}"));
                record.record(&format!("/{index}/rights"), &format!("/{index}/{title}"));
                record.record(&format!("/{index}/rightsUri"), &format!("/{index}/path"));
                record.record(
                    &format!("/{index}/rightsIdentifier"),
                    &format!("/{index}/name"),
                );
            }

            Value::Object(rights)
        })
        .collect()
}
