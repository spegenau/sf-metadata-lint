use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecordAlertDataSource  {
	#[serde(rename = "apexClass")]
	pub apex_class: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
