use crate::metadata::PermissionSetTabVisibility::PermissionSetTabVisibility;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PermissionSetTabSetting  {
	#[serde(rename = "tab")]
	pub tab: String,
	#[serde(rename = "visibility")]
	pub visibility: PermissionSetTabVisibility,
}
