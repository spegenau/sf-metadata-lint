use crate::metadata::CleanRuleStatus::CleanRuleStatus;
use crate::metadata::FieldMapping::FieldMapping;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CleanRule  {
	#[serde(rename = "bulkEnabled")]
	pub bulk_enabled: bool,
	#[serde(rename = "bypassTriggers")]
	pub bypass_triggers: bool,
	#[serde(rename = "bypassWorkflow")]
	pub bypass_workflow: bool,
	#[serde(rename = "description")]
	pub description: String,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "fieldMappings")]
	pub field_mappings: Option<Vec<FieldMapping>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "matchRule")]
	pub match_rule: String,
	#[serde(rename = "sourceSobjectType")]
	pub source_sobject_type: String,
	#[serde(rename = "status")]
	pub status: CleanRuleStatus,
	#[serde(rename = "targetSobjectType")]
	pub target_sobject_type: String,
}
