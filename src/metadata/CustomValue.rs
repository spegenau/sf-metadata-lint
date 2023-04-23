use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomValue  {
	#[serde(rename = "color")]
	pub color: Option<String>,
	#[serde(rename = "default")]
	pub default: bool,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "label")]
	pub label: Option<String>,
}
