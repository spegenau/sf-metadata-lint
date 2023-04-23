use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExperienceResource  {
	#[serde(rename = "fileName")]
	pub file_name: String,
	#[serde(rename = "format")]
	pub format: String,
	#[serde(rename = "source")]
	pub source: Option<String>,
	#[serde(rename = "type")]
	pub _type: String,
}
