use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct InstalledPackage  {
	#[serde(rename = "activateRSS")]
	pub activate_rss: bool,
	#[serde(rename = "password")]
	pub password: Option<String>,
	#[serde(rename = "securityType")]
	pub security_type: Option<String>,
	#[serde(rename = "versionNumber")]
	pub version_number: String,
}
