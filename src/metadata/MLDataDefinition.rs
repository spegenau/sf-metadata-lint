use crate::metadata::MLDataDefinitionType::MLDataDefinitionType;
use crate::metadata::MLField::MLField;
use crate::metadata::MLFilter::MLFilter;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MLDataDefinition  {
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "entityDeveloperName")]
	pub entity_developer_name: String,
	#[serde(rename = "excludedFields")]
	pub excluded_fields: Option<Vec<String>>,
	#[serde(rename = "includedFields")]
	pub included_fields: Option<Vec<String>>,
	#[serde(rename = "joinFields")]
	pub join_fields: Option<Vec<MLField>>,
	#[serde(rename = "parentDefinitionDevName")]
	pub parent_definition_dev_name: String,
	#[serde(rename = "scoringFilter")]
	pub scoring_filter: Option<MLFilter>,
	#[serde(rename = "segmentFilter")]
	pub segment_filter: Option<MLFilter>,
	#[serde(rename = "trainingFilter")]
	pub training_filter: Option<MLFilter>,
	#[serde(rename = "type")]
	pub _type: MLDataDefinitionType,
}
