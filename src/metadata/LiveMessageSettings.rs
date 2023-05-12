use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LiveMessageSettings  {
	#[serde(rename = "enableCheckCEUserPerm")]
	pub enable_check_ce_user_perm: Option<bool>,
	#[serde(rename = "enableLiveMessage")]
	pub enable_live_message: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
