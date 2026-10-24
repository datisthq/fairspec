use fairspec_metadata::FileDialect;

/// Python's union of the dialects with `headerRows` and `commentRows`: csv, tsv, json,
/// jsonl, xlsx and ods. Other variants read as having neither.
pub type FileDialectWithHeaderAndCommentRows = FileDialect;
