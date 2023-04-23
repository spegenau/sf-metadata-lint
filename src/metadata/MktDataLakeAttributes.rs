use crate::metadata::DefinitionCreationType::DefinitionCreationType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MktDataLakeAttributes  {
	#[serde(rename = "creationType")]
	pub creation_type: Option<DefinitionCreationType>,
	#[serde(rename = "isEnabled")]
	pub is_enabled: Option<bool>,
	#[serde(rename = "objectCategory")]
	pub object_category: Option<String>,
}
