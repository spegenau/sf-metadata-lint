use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ActionsSettings  {
	#[serde(rename = "enableDefaultQuickActionsOn")]
	pub enable_default_quick_actions_on: Option<bool>,
	#[serde(rename = "enableMdpEnabled")]
	pub enable_mdp_enabled: Option<bool>,
	#[serde(rename = "enableOfflineWebLinks")]
	pub enable_offline_web_links: Option<bool>,
	#[serde(rename = "enableThirdPartyActions")]
	pub enable_third_party_actions: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
