use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PicklistEntry  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "defaultValue")]
	pub default_value: bool,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "validFor")]
	pub valid_for: Option<String>,
	#[serde(rename = "value")]
	pub value: String,
}
