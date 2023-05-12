use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowDefinition  {
	#[serde(rename = "activeVersionNumber")]
	pub active_version_number: Option<i32>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
