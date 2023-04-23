use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LicensedCustomPermissions  {
	#[serde(rename = "customPermission")]
	pub custom_permission: String,
	#[serde(rename = "licenseDefinition")]
	pub license_definition: String,
}
