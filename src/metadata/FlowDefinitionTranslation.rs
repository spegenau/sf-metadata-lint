use crate::metadata::FlowTranslation::FlowTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowDefinitionTranslation  {
	#[serde(rename = "flows")]
	pub flows: Option<Vec<FlowTranslation>>,
	#[serde(rename = "fullName")]
	pub full_name: String,
	#[serde(rename = "label")]
	pub label: Option<String>,
}
