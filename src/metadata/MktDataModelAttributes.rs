use crate::metadata::DefinitionCreationType::DefinitionCreationType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MktDataModelAttributes  {
	#[serde(rename = "creationType")]
	pub creation_type: Option<DefinitionCreationType>,
	#[serde(rename = "dataModelTaxonomy")]
	pub data_model_taxonomy: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isEnabled")]
	pub is_enabled: Option<bool>,
	#[serde(rename = "isSegmentable")]
	pub is_segmentable: Option<bool>,
	#[serde(rename = "isUsedForMetrics")]
	pub is_used_for_metrics: Option<bool>,
	#[serde(rename = "objectCategory")]
	pub object_category: Option<String>,
	#[serde(rename = "referenceEntityGroup")]
	pub reference_entity_group: Option<String>,
	#[serde(rename = "referenceEntityName")]
	pub reference_entity_name: Option<String>,
	#[serde(rename = "referenceEntitySubjectArea")]
	pub reference_entity_subject_area: Option<String>,
}
