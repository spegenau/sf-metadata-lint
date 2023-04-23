use crate::metadata::FolderAccessTypes::FolderAccessTypes;
use crate::metadata::FolderShare::FolderShare;
use crate::metadata::PublicFolderAccess::PublicFolderAccess;
use crate::metadata::SharedTo::SharedTo;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Folder  {
	#[serde(rename = "accessType")]
	pub access_type: Option<FolderAccessTypes>,
	#[serde(rename = "folderShares")]
	pub folder_shares: Option<Vec<FolderShare>>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "publicFolderAccess")]
	pub public_folder_access: Option<PublicFolderAccess>,
	#[serde(rename = "sharedTo")]
	pub shared_to: Option<SharedTo>,
}
