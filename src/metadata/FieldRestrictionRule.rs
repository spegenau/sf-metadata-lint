use crate::metadata::ClassificationType::ClassificationType;
use crate::metadata::EnforcementType::EnforcementType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FieldRestrictionRule  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "classification")]
	pub classification: Option<Vec<String>>,
	#[serde(rename = "classificationType")]
	pub classification_type: Option<ClassificationType>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "enforcementType")]
	pub enforcement_type: EnforcementType,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "recordFilter")]
	pub record_filter: String,
	#[serde(rename = "targetEntity")]
	pub target_entity: String,
	#[serde(rename = "userCriteria")]
	pub user_criteria: String,
	#[serde(rename = "version")]
	pub version: i32,
}
