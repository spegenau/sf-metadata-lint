use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DelegateGroup  {
	#[serde(rename = "customObjects")]
	pub custom_objects: Option<Vec<String>>,
	#[serde(rename = "groups")]
	pub groups: Option<Vec<String>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "loginAccess")]
	pub login_access: bool,
	#[serde(rename = "permissionSets")]
	pub permission_sets: Option<Vec<String>>,
	#[serde(rename = "profiles")]
	pub profiles: Option<Vec<String>>,
	#[serde(rename = "roles")]
	pub roles: Option<Vec<String>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
