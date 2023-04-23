use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PermissionSetRecordTypeVisibility  {
	#[serde(rename = "recordType")]
	pub record_type: String,
	#[serde(rename = "visible")]
	pub visible: bool,
}
