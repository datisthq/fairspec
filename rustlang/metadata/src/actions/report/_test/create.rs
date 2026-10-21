use crate::actions::report::create::create_report;
use crate::models::error::FairspecError;
use crate::models::error::metadata::MetadataError;

fn create_errors(count: usize) -> Vec<FairspecError> {
    (0..count)
        .map(|index| {
            FairspecError::Metadata(MetadataError {
                message: format!("error{index}"),
                json_pointer: format!("/{index}"),
                ..MetadataError::default()
            })
        })
        .collect()
}

#[test]
fn test_valid_report_with_no_errors() {
    let report = create_report(Vec::new(), None);
    assert!(report.valid);
    assert!(report.errors.is_empty());
}

#[test]
fn test_invalid_report_with_errors() {
    let report = create_report(create_errors(2), None);
    assert!(!report.valid);
    assert_eq!(report.errors.len(), 2);
}

#[test]
fn test_max_errors_limits_errors() {
    let report = create_report(create_errors(10), Some(3));
    assert!(!report.valid);
    assert_eq!(report.errors.len(), 3);
}
