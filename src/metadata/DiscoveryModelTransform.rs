use crate::metadata::DiscoveryAIModelTransformationType::DiscoveryAIModelTransformationType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryModelTransform  {
	#[serde(rename = "config")]
	pub config: Option<String>,
	#[serde(rename = "sourceFieldNames")]
	pub source_field_names: Option<Vec<String>>,
	#[serde(rename = "targetFieldNames")]
	pub target_field_names: Option<Vec<String>>,
	#[serde(rename = "type")]
	pub _type: DiscoveryAIModelTransformationType,
}
