use fairspec_metadata::Result;
use polars::sql::SQLContext;

use crate::helpers::error::PolarsResultExt;
use crate::models::table::Table;

/// Runs a SQL query against a table, exposed as `self`.
pub fn query_table(table: Table, query: &str) -> Result<Table> {
    query_tables([("self".to_string(), table)], query)
}

/// Runs a SQL query against several tables, each exposed under its name.
pub fn query_tables(
    tables: impl IntoIterator<Item = (String, Table)>,
    query: &str,
) -> Result<Table> {
    let mut context = SQLContext::new();
    for (name, table) in tables {
        context.register(&name, table);
    }
    context.execute(query).into_result()
}
