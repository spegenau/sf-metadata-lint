use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportTypeColumn  {
	#[serde(rename = "checkedByDefault")]
	pub checked_by_default: bool,
	#[serde(rename = "displayNameOverride")]
	pub display_name_override: Option<String>,
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "table")]
	pub table: String,
}
