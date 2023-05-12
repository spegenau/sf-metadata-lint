use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ArchiveSettings  {
	#[serde(rename = "enableEntityArchivingEnabled")]
	pub enable_entity_archiving_enabled: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
