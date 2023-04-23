use crate::metadata::ManageableState::ManageableState;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FileProperties  {
	#[serde(rename = "createdById")]
	pub created_by_id: String,
	#[serde(rename = "createdByName")]
	pub created_by_name: String,
	#[serde(rename = "createdDate")]
	pub created_date: String,
	#[serde(rename = "fileName")]
	pub file_name: String,
	#[serde(rename = "fullName")]
	pub full_name: String,
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "lastModifiedById")]
	pub last_modified_by_id: String,
	#[serde(rename = "lastModifiedByName")]
	pub last_modified_by_name: String,
	#[serde(rename = "lastModifiedDate")]
	pub last_modified_date: String,
	#[serde(rename = "manageableState")]
	pub manageable_state: Option<ManageableState>,
	#[serde(rename = "namespacePrefix")]
	pub namespace_prefix: Option<String>,
	#[serde(rename = "type")]
	pub _type: String,
}
