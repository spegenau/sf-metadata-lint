use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PathAssistantSettings  {
	#[serde(rename = "canOverrideAutoPathCollapseWithUserPref")]
	pub can_override_auto_path_collapse_with_user_pref: Option<bool>,
	#[serde(rename = "pathAssistantEnabled")]
	pub path_assistant_enabled: Option<bool>,
}
