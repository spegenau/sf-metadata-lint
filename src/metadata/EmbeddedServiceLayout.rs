use crate::metadata::EmbeddedServiceLayoutRule::EmbeddedServiceLayoutRule;
use crate::metadata::EmbeddedServiceLayoutType::EmbeddedServiceLayoutType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceLayout  {
	#[serde(rename = "embeddedServiceLayoutRules")]
	pub embedded_service_layout_rules: Option<Vec<EmbeddedServiceLayoutRule>>,
	#[serde(rename = "layout")]
	pub layout: String,
	#[serde(rename = "layoutType")]
	pub layout_type: Option<EmbeddedServiceLayoutType>,
}
