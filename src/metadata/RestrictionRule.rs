use crate::metadata::EnforcementType::EnforcementType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RestrictionRule  {
	#[serde(rename = "active")]
	pub active: bool,
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
