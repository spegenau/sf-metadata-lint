use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FieldServiceMobileExtension  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "fileName")]
	pub file_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: Option<String>,
	#[serde(rename = "size")]
	pub size: Option<i32>,
	#[serde(rename = "version")]
	pub version: Option<i32>,
}
