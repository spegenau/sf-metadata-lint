use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DuplicateRuleFilterItem  {
	#[serde(rename = "sortOrder")]
	pub sort_order: i32,
	#[serde(rename = "table")]
	pub table: String,
}
