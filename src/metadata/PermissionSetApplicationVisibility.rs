use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PermissionSetApplicationVisibility  {
	#[serde(rename = "application")]
	pub application: String,
	#[serde(rename = "visible")]
	pub visible: bool,
}
