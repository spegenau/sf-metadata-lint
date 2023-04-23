use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PermissionSetLicenseDefinitionCustomPermission  {
	#[serde(rename = "name")]
	pub name: String,
}
