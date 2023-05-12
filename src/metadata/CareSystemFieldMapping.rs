use crate::metadata::SourceSystemFieldRole::SourceSystemFieldRole;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CareSystemFieldMapping  {
	#[serde(rename = "externalIdField")]
	pub external_id_field: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "role")]
	pub role: SourceSystemFieldRole,
	#[serde(rename = "sourceSystem")]
	pub source_system: Option<String>,
	#[serde(rename = "targetObject")]
	pub target_object: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
