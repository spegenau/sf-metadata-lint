use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DigitalExperience  {
	#[serde(rename = "fileName")]
	pub file_name: String,
	#[serde(rename = "filePath")]
	pub file_path: Option<String>,
	#[serde(rename = "format")]
	pub format: String,
}
