use crate::metadata::DiscoveryCustomPrescribableFieldDefinition::DiscoveryCustomPrescribableFieldDefinition;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryPrescribableField  {
	#[serde(rename = "customDefinitions")]
	pub custom_definitions: Option<Vec<DiscoveryCustomPrescribableFieldDefinition>>,
	#[serde(rename = "name")]
	pub name: String,
}
