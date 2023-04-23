use crate::metadata::BotVersionTranslation::BotVersionTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotTranslation  {
	#[serde(rename = "botVersions")]
	pub bot_versions: Option<Vec<BotVersionTranslation>>,
	#[serde(rename = "fullName")]
	pub full_name: String,
}
