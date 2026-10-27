use std::collections::HashMap;

use serde_json::Value;

use crate::models::error::error::FairspecError;

/// How a converted Frictionless descriptor maps back to the one the user wrote
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FrictionlessMapping {
    pub pointers: Vec<FrictionlessPointer>,
    pub resource_names: HashMap<String, String>,
}

/// A Fairspec pointer written from a Frictionless one; a verbatim value was copied as it
/// is, so pointers below it map key for key
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FrictionlessPointer {
    pub fairspec: String,
    pub frictionless: String,
    pub verbatim: bool,
}

/// Records pointer pairs under a prefix; without pointers to write to, it records nothing
#[derive(Debug, Default)]
pub struct FrictionlessRecorder<'a> {
    pointers: Option<&'a mut Vec<FrictionlessPointer>>,
    fairspec: String,
    frictionless: String,
}

impl<'a> FrictionlessRecorder<'a> {
    pub fn new(
        pointers: &'a mut Vec<FrictionlessPointer>,
        fairspec: &str,
        frictionless: &str,
    ) -> Self {
        Self {
            pointers: Some(pointers),
            fairspec: fairspec.to_string(),
            frictionless: frictionless.to_string(),
        }
    }

    pub fn record(&mut self, fairspec: &str, frictionless: &str) {
        self.push_pointer(fairspec, frictionless, false);
    }

    pub fn record_verbatim(&mut self, fairspec: &str, frictionless: &str) {
        self.push_pointer(fairspec, frictionless, true);
    }

    pub fn nest(&mut self, fairspec: &str, frictionless: &str) -> FrictionlessRecorder<'_> {
        FrictionlessRecorder {
            pointers: self.pointers.as_deref_mut(),
            fairspec: format!("{}{fairspec}", self.fairspec),
            frictionless: format!("{}{frictionless}", self.frictionless),
        }
    }

    fn push_pointer(&mut self, fairspec: &str, frictionless: &str, verbatim: bool) {
        if let Some(pointers) = self.pointers.as_deref_mut() {
            pointers.push(FrictionlessPointer {
                fairspec: format!("{}{fairspec}", self.fairspec),
                frictionless: format!("{}{frictionless}", self.frictionless),
                verbatim,
            });
        }
    }
}

/// Rewrite errors about a converted descriptor to point at the Frictionless one
pub fn map_frictionless_errors(
    errors: Vec<FairspecError>,
    mapping: &FrictionlessMapping,
) -> Vec<FairspecError> {
    errors
        .into_iter()
        .map(|error| map_frictionless_error(error, mapping))
        .collect()
}

// Every error variant is a JSON object; going through JSON spares a match per variant
fn map_frictionless_error(error: FairspecError, mapping: &FrictionlessMapping) -> FairspecError {
    let Ok(mut value) = serde_json::to_value(&error) else {
        return error;
    };
    let names = &mapping.resource_names;

    if let Some(Value::String(pointer)) = value.get_mut("jsonPointer") {
        *pointer = map_frictionless_pointer(pointer, mapping);
    }

    if let Some(Value::String(name)) = value.get_mut("resourceName")
        && let Some(original) = names.get(name.as_str())
    {
        *name = original.clone();
    }

    if value["type"] == "foreignKey"
        && let Some(Value::String(resource)) = value.pointer_mut("/foreignKey/reference/resource")
        && let Some(original) = names.get(resource.as_str())
    {
        *resource = original.clone();
    }

    serde_json::from_value(value).unwrap_or(error)
}

fn map_frictionless_pointer(pointer: &str, mapping: &FrictionlessMapping) -> String {
    let mut best: Option<&FrictionlessPointer> = None;
    for entry in &mapping.pointers {
        let is_prefix = pointer == entry.fairspec
            || pointer
                .strip_prefix(entry.fairspec.as_str())
                .is_some_and(|rest| rest.starts_with('/'));
        if is_prefix && best.is_none_or(|best| entry.fairspec.len() > best.fairspec.len()) {
            best = Some(entry);
        }
    }

    match best {
        None => pointer.to_string(),
        Some(best) if !best.verbatim => best.frictionless.clone(),
        Some(best) => format!("{}{}", best.frictionless, &pointer[best.fairspec.len()..]),
    }
}
