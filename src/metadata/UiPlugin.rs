use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UiPlugin  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "extensionPointIdentifier")]
	pub extension_point_identifier: String,
	#[serde(rename = "isEnabled")]
	pub is_enabled: bool,
	#[serde(rename = "language")]
	pub language: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
