mod actions;
mod helpers;
mod models;
mod plugin;
mod plugins;
mod schemas;
mod settings;
mod utils;

pub use polars;

// models
pub use models::cell::CellMapping;
pub use models::check::{SqlTableCheck, TableCheck, TableCheckCondition};
pub use models::column::{ColumnMapping, DenormalizeColumnOptions, PolarsColumn};
pub use models::data::{DataRecord, DataRow};
pub use models::file_dialect::FileDialectWithHeaderAndCommentRows;
pub use models::frame::Frame;
pub use models::schema::{
    InferTableSchemaOptions, PolarsSchema, SchemaMapping, TableSchemaOptions,
};
pub use models::table::{LoadTableOptions, SaveTableOptions, Table};
pub use plugin::TablePlugin;

// actions
pub use actions::check::inspect::inspect_table_checks;
pub use actions::data::file_dialect::get_records_from_rows;
pub use actions::table::denormalize::denormalize_table;
pub use actions::table::inspect::inspect_table;
pub use actions::table::normalize::normalize_table;
pub use actions::table::query::{query_table, query_tables};
pub use actions::table_schema::infer::{
    infer_table_schema_from_sample, infer_table_schema_from_table,
};
pub use actions::table_schema::pattern::inspect_table_schema_patterns;

// plugins
pub use plugins::arrow::actions::table::load::load_arrow_table;
pub use plugins::arrow::actions::table::save::save_arrow_table;
pub use plugins::arrow::plugin::ArrowPlugin;
pub use plugins::csv::actions::file_dialect::infer::infer_csv_file_dialect;
pub use plugins::csv::actions::table::load::load_csv_table;
pub use plugins::csv::actions::table::save::save_csv_table;
pub use plugins::csv::plugin::CsvPlugin;
pub use plugins::inline::actions::table::load::load_inline_table;
pub use plugins::inline::plugin::InlinePlugin;
pub use plugins::json::actions::file_dialect::infer::infer_json_file_dialect;
pub use plugins::json::actions::table::load::load_json_table;
pub use plugins::json::actions::table::save::save_json_table;
pub use plugins::json::plugin::JsonPlugin;
pub use plugins::parquet::actions::table::load::load_parquet_table;
pub use plugins::parquet::actions::table::save::save_parquet_table;
pub use plugins::parquet::plugin::ParquetPlugin;
pub use plugins::sqlite::actions::table::load::load_sqlite_table;
pub use plugins::sqlite::actions::table::save::save_sqlite_table;
pub use plugins::sqlite::plugin::SqlitePlugin;
pub use plugins::xlsx::actions::file_dialect::infer::infer_xlsx_file_dialect;
pub use plugins::xlsx::actions::table::load::load_xlsx_table;
pub use plugins::xlsx::actions::table::save::save_xlsx_table;
pub use plugins::xlsx::plugin::XlsxPlugin;
pub use settings::QUERY_ENGINE;
