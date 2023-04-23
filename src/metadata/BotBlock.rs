use crate::metadata::BotBlockVersion::BotBlockVersion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotBlock  {
	#[serde(rename = "botBlockVersions")]
	pub bot_block_versions: Option<Vec<BotBlockVersion>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "icon")]
	pub icon: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "richContentEnabled")]
	pub rich_content_enabled: Option<bool>,
}
