use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileObjectPermissions  {
	#[serde(rename = "allowCreate")]
	pub allow_create: Option<bool>,
	#[serde(rename = "allowDelete")]
	pub allow_delete: Option<bool>,
	#[serde(rename = "allowEdit")]
	pub allow_edit: Option<bool>,
	#[serde(rename = "allowRead")]
	pub allow_read: Option<bool>,
	#[serde(rename = "modifyAllRecords")]
	pub modify_all_records: Option<bool>,
	#[serde(rename = "object")]
	pub object: String,
	#[serde(rename = "viewAllRecords")]
	pub view_all_records: Option<bool>,
}
