use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotSettings  {
	#[serde(rename = "enableBots")]
	pub enable_bots: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
