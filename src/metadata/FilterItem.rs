use crate::metadata::FilterOperation::FilterOperation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FilterItem  {
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "operation")]
	pub operation: FilterOperation,
	#[serde(rename = "value")]
	pub value: Option<String>,
	#[serde(rename = "valueField")]
	pub value_field: Option<String>,
}
