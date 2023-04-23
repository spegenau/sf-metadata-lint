use crate::metadata::EmbeddedServiceComponentBundleType::EmbeddedServiceComponentBundleType;
use crate::metadata::EmbeddedServiceCustomComponentType::EmbeddedServiceCustomComponentType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceCustomComponent  {
	#[serde(rename = "componentBundleType")]
	pub component_bundle_type: Option<EmbeddedServiceComponentBundleType>,
	#[serde(rename = "customComponent")]
	pub custom_component: Option<String>,
	#[serde(rename = "customComponentType")]
	pub custom_component_type: Option<EmbeddedServiceCustomComponentType>,
}
