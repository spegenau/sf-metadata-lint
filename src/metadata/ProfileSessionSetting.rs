use crate::metadata::SessionSecurityLevel::SessionSecurityLevel;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileSessionSetting  {
	#[serde(rename = "externalCommunityUserIdentityVerif")]
	pub external_community_user_identity_verif: bool,
	#[serde(rename = "forceLogout")]
	pub force_logout: bool,
	#[serde(rename = "profile")]
	pub profile: String,
	#[serde(rename = "requiredSessionLevel")]
	pub required_session_level: Option<SessionSecurityLevel>,
	#[serde(rename = "sessionPersistence")]
	pub session_persistence: bool,
	#[serde(rename = "sessionTimeout")]
	pub session_timeout: i32,
	#[serde(rename = "sessionTimeoutWarning")]
	pub session_timeout_warning: bool,
}
