use crate::metadata::MCNodeType::MCNodeType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ManagedContentNodeType  {
	#[serde(rename = "helpText")]
	pub help_text: Option<String>,
	#[serde(rename = "isLocalizable")]
	pub is_localizable: Option<bool>,
	#[serde(rename = "isRequired")]
	pub is_required: Option<bool>,
	#[serde(rename = "nodeLabel")]
	pub node_label: String,
	#[serde(rename = "nodeName")]
	pub node_name: String,
	#[serde(rename = "nodeType")]
	pub node_type: MCNodeType,
	#[serde(rename = "placeholderText")]
	pub placeholder_text: Option<String>,
}
