use crate::metadata::ContentAssetAccess::ContentAssetAccess;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ContentAssetLink  {
	#[serde(rename = "access")]
	pub access: ContentAssetAccess,
	#[serde(rename = "isManagingWorkspace")]
	pub is_managing_workspace: Option<bool>,
	#[serde(rename = "name")]
	pub name: Option<String>,
}
