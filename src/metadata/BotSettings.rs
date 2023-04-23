use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotSettings  {
	#[serde(rename = "enableBots")]
	pub enable_bots: Option<bool>,
}
