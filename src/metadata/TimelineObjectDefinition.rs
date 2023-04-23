use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct TimelineObjectDefinition  {
	#[serde(rename = "baseObject")]
	pub base_object: String,
	#[serde(rename = "definition")]
	pub definition: String,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
