use crate::metadata::ManagedContentNodeType::ManagedContentNodeType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ManagedContentType  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "managedContentNodeTypes")]
	pub managed_content_node_types: Option<Vec<ManagedContentNodeType>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
