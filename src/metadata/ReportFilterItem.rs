use crate::metadata::FilterOperation::FilterOperation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportFilterItem  {
	#[serde(rename = "column")]
	pub column: String,
	#[serde(rename = "columnToColumn")]
	pub column_to_column: Option<bool>,
	#[serde(rename = "isUnlocked")]
	pub is_unlocked: Option<bool>,
	#[serde(rename = "operator")]
	pub operator: FilterOperation,
	#[serde(rename = "snapshot")]
	pub snapshot: Option<String>,
	#[serde(rename = "value")]
	pub value: Option<String>,
}
