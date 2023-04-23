use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommandActionParam  {
	#[serde(rename = "defaultValue")]
	pub default_value: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "required")]
	pub required: Option<bool>,
	#[serde(rename = "type")]
	pub _type: String,
}
