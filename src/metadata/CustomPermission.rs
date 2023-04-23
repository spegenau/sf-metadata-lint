use crate::metadata::CustomPermissionDependencyRequired::CustomPermissionDependencyRequired;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomPermission  {
	#[serde(rename = "connectedApp")]
	pub connected_app: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isLicensed")]
	pub is_licensed: bool,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "requiredPermission")]
	pub required_permission: Option<Vec<CustomPermissionDependencyRequired>>,
}
