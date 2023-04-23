use crate::metadata::EntitlementProcessMilestoneItem::EntitlementProcessMilestoneItem;
use crate::metadata::FilterItem::FilterItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EntitlementProcess  {
	#[serde(rename = "SObjectType")]
	pub s_object_type: Option<String>,
	#[serde(rename = "active")]
	pub active: Option<bool>,
	#[serde(rename = "businessHours")]
	pub business_hours: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "entryStartDateField")]
	pub entry_start_date_field: Option<String>,
	#[serde(rename = "exitCriteriaBooleanFilter")]
	pub exit_criteria_boolean_filter: Option<String>,
	#[serde(rename = "exitCriteriaFilterItems")]
	pub exit_criteria_filter_items: Option<Vec<FilterItem>>,
	#[serde(rename = "exitCriteriaFormula")]
	pub exit_criteria_formula: Option<String>,
	#[serde(rename = "isRecordTypeApplied")]
	pub is_record_type_applied: Option<bool>,
	#[serde(rename = "isVersionDefault")]
	pub is_version_default: Option<bool>,
	#[serde(rename = "milestones")]
	pub milestones: Option<Vec<EntitlementProcessMilestoneItem>>,
	#[serde(rename = "name")]
	pub name: Option<String>,
	#[serde(rename = "recordType")]
	pub record_type: Option<String>,
	#[serde(rename = "versionMaster")]
	pub version_master: Option<String>,
	#[serde(rename = "versionNotes")]
	pub version_notes: Option<String>,
	#[serde(rename = "versionNumber")]
	pub version_number: Option<i32>,
}
