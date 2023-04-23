use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PermissionSetApexClassAccess  {
	#[serde(rename = "apexClass")]
	pub apex_class: String,
	#[serde(rename = "enabled")]
	pub enabled: bool,
}
