use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LiveAgentSettings  {
	#[serde(rename = "enableChatFindOrCreateEnable")]
	pub enable_chat_find_or_create_enable: Option<bool>,
	#[serde(rename = "enableLiveAgent")]
	pub enable_live_agent: Option<bool>,
	#[serde(rename = "enableQuickTextEnabled")]
	pub enable_quick_text_enabled: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
