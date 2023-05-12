use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Territory2Type  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "priority")]
	pub priority: i32,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
