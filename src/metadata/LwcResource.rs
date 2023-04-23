use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LwcResource  {
	#[serde(rename = "filePath")]
	pub file_path: String,
	#[serde(rename = "source")]
	pub source: String,
}
