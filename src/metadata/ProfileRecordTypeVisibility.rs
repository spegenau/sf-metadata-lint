use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileRecordTypeVisibility  {
	#[serde(rename = "default")]
	pub default: bool,
	#[serde(rename = "personAccountDefault")]
	pub person_account_default: Option<bool>,
	#[serde(rename = "recordType")]
	pub record_type: String,
	#[serde(rename = "visible")]
	pub visible: bool,
}
