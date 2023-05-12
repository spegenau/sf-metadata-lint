use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PortalDelegablePermissionSet  {
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "permissionSet")]
	pub permission_set: String,
	#[serde(rename = "profile")]
	pub profile: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
