use serde::{Deserialize, Serialize};

use super::cell::CellError;
use super::check::{CheckError, TableCheckError};
use super::column::ColumnError;
use super::foreign_key::ForeignKeyError;
use super::row::RowError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TableError {
    Column(ColumnError),
    Row(RowError),
    Cell(CellError),
    ForeignKey(ForeignKeyError),
    TableCheck(TableCheckError),
    Check(CheckError),
}
