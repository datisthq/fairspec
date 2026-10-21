use crate::models::error::FairspecError;
use crate::models::report::Report;

pub fn create_report(mut errors: Vec<FairspecError>, max_errors: Option<usize>) -> Report {
    if let Some(max_errors) = max_errors {
        errors.truncate(max_errors);
    }
    Report {
        valid: errors.is_empty(),
        errors,
    }
}
