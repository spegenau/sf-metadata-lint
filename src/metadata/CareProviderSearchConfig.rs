use crate::metadata::ProviderSearchObjectMapping::ProviderSearchObjectMapping;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CareProviderSearchConfig  {
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "mappedObject")]
	pub mapped_object: ProviderSearchObjectMapping,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "sourceField")]
	pub source_field: Option<String>,
	#[serde(rename = "targetField")]
	pub target_field: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
