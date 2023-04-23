use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MobileSecurityAssignment  {
	#[serde(rename = "connectedApplication")]
	pub connected_application: Option<String>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "profile")]
	pub profile: Option<String>,
}
