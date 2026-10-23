use fairspec_library::fairspec_metadata::{Error, Result};

/// The bounds an input schema states, which serde alone does not check.
pub fn check_range(name: &str, value: Option<usize>, minimum: usize, maximum: usize) -> Result<()> {
    match value {
        Some(value) if value < minimum => Err(Error::Parse(format!(
            "{name}: Input should be greater than or equal to {minimum}"
        ))),
        Some(value) if value > maximum => Err(Error::Parse(format!(
            "{name}: Input should be less than or equal to {maximum}"
        ))),
        _ => Ok(()),
    }
}
