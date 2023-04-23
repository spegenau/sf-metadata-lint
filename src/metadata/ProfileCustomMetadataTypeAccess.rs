use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileCustomMetadataTypeAccess  {
	#[serde(rename = "enabled")]
	pub enabled: bool,
	#[serde(rename = "name")]
	pub name: String,
}
