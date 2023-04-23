use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PermissionSetObjectPermissions  {
	#[serde(rename = "allowCreate")]
	pub allow_create: bool,
	#[serde(rename = "allowDelete")]
	pub allow_delete: bool,
	#[serde(rename = "allowEdit")]
	pub allow_edit: bool,
	#[serde(rename = "allowRead")]
	pub allow_read: bool,
	#[serde(rename = "modifyAllRecords")]
	pub modify_all_records: bool,
	#[serde(rename = "object")]
	pub object: String,
	#[serde(rename = "viewAllRecords")]
	pub view_all_records: bool,
}
