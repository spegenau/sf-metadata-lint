use crate::metadata::FolderShareAccessLevel::FolderShareAccessLevel;
use crate::metadata::FolderSharedToType::FolderSharedToType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FolderShare  {
	#[serde(rename = "accessLevel")]
	pub access_level: FolderShareAccessLevel,
	#[serde(rename = "sharedTo")]
	pub shared_to: String,
	#[serde(rename = "sharedToType")]
	pub shared_to_type: FolderSharedToType,
}
