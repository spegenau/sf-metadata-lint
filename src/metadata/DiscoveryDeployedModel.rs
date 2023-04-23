use crate::metadata::DiscoveryFieldMap::DiscoveryFieldMap;
use crate::metadata::DiscoveryFilter::DiscoveryFilter;
use crate::metadata::DiscoveryPrescribableField::DiscoveryPrescribableField;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryDeployedModel  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "aiModel")]
	pub ai_model: String,
	#[serde(rename = "classificationThreshold")]
	pub classification_threshold: Option<f32>,
	#[serde(rename = "fieldMappings")]
	pub field_mappings: Option<Vec<DiscoveryFieldMap>>,
	#[serde(rename = "filters")]
	pub filters: Option<Vec<DiscoveryFilter>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "prescribableFields")]
	pub prescribable_fields: Option<Vec<DiscoveryPrescribableField>>,
}
