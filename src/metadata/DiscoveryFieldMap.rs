use crate::metadata::DiscoveryFieldMapSourceType::DiscoveryFieldMapSourceType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryFieldMap  {
	#[serde(rename = "mappedField")]
	pub mapped_field: String,
	#[serde(rename = "modelField")]
	pub model_field: String,
	#[serde(rename = "sobjectFieldJoinKey")]
	pub sobject_field_join_key: Option<String>,
	#[serde(rename = "source")]
	pub source: Option<String>,
	#[serde(rename = "sourceFieldJoinKey")]
	pub source_field_join_key: Option<String>,
	#[serde(rename = "sourceType")]
	pub source_type: DiscoveryFieldMapSourceType,
}
