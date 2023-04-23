use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PermissionSetGroup  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "hasActivationRequired")]
	pub has_activation_required: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "mutingPermissionSets")]
	pub muting_permission_sets: Option<Vec<String>>,
	#[serde(rename = "permissionSets")]
	pub permission_sets: Option<Vec<String>>,
	#[serde(rename = "status")]
	pub status: Option<String>,
}
