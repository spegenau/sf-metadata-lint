use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConnectedAppSessionPolicy  {
	#[serde(rename = "policyAction")]
	pub policy_action: Option<String>,
	#[serde(rename = "sessionLevel")]
	pub session_level: Option<String>,
	#[serde(rename = "sessionTimeout")]
	pub session_timeout: Option<i32>,
}
