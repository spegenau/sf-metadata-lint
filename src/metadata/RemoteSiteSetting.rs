use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RemoteSiteSetting  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "disableProtocolSecurity")]
	pub disable_protocol_security: bool,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "url")]
	pub url: String,
}
