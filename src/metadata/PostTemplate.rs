use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PostTemplate  {
	#[serde(rename = "default")]
	pub default: Option<bool>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "fields")]
	pub fields: Option<Vec<String>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
