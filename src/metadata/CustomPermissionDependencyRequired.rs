use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomPermissionDependencyRequired  {
	#[serde(rename = "customPermission")]
	pub custom_permission: String,
	#[serde(rename = "dependency")]
	pub dependency: bool,
}
