use std::fmt;

use serde::{Deserialize, Serialize};

pub const DATE_FORMAT: &str = "YYYY-MM-DD";
pub const TIME_FORMAT: &str = "HH:mm";
pub const IS_TIME_12HR: bool = true;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BaseId(String);

impl From<String> for BaseId {
    fn from(value: String) -> Self {
        BaseId(value)
    }
}

impl fmt::Display for BaseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TableId(String);

impl fmt::Display for TableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ColumnId(String);

impl fmt::Display for ColumnId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ViewId(String);

impl fmt::Display for ViewId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
