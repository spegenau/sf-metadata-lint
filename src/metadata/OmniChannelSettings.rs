use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OmniChannelSettings  {
	#[serde(rename = "enableOmniAutoLoginPrompt")]
	pub enable_omni_auto_login_prompt: Option<bool>,
	#[serde(rename = "enableOmniChannel")]
	pub enable_omni_channel: Option<bool>,
	#[serde(rename = "enableOmniSecondaryRoutingPriority")]
	pub enable_omni_secondary_routing_priority: Option<bool>,
	#[serde(rename = "enableOmniSkillsRouting")]
	pub enable_omni_skills_routing: Option<bool>,
}
