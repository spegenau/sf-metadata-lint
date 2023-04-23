use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CMSConnectResourceDefinition  {
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "options")]
	pub options: i32,
	#[serde(rename = "payloadType")]
	pub payload_type: String,
	#[serde(rename = "resourceIdPath")]
	pub resource_id_path: Option<String>,
	#[serde(rename = "resourceNamePath")]
	pub resource_name_path: Option<String>,
	#[serde(rename = "resourcePath")]
	pub resource_path: String,
	#[serde(rename = "rootNodePath")]
	pub root_node_path: Option<String>,
}
