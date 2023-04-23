use crate::metadata::ExternalDataSrcDescSubtype::ExternalDataSrcDescSubtype;
use crate::metadata::ExternalDataSrcDescType::ExternalDataSrcDescType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExternalDataSrcDescriptor  {
	#[serde(rename = "customObject")]
	pub custom_object: Option<String>,
	#[serde(rename = "descriptor")]
	pub descriptor: String,
	#[serde(rename = "descriptorVersion")]
	pub descriptor_version: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "externalDataSource")]
	pub external_data_source: String,
	#[serde(rename = "subtype")]
	pub subtype: ExternalDataSrcDescSubtype,
	#[serde(rename = "systemVersion")]
	pub system_version: i32,
	#[serde(rename = "type")]
	pub _type: ExternalDataSrcDescType,
}
