use crate::metadata::DupeActionType::DupeActionType;
use crate::metadata::DupeSecurityOptionType::DupeSecurityOptionType;
use crate::metadata::DuplicateRuleFilter::DuplicateRuleFilter;
use crate::metadata::DuplicateRuleMatchRule::DuplicateRuleMatchRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DuplicateRule  {
	#[serde(rename = "actionOnInsert")]
	pub action_on_insert: DupeActionType,
	#[serde(rename = "actionOnUpdate")]
	pub action_on_update: DupeActionType,
	#[serde(rename = "alertText")]
	pub alert_text: String,
	#[serde(rename = "description")]
	pub description: String,
	#[serde(rename = "duplicateRuleFilter")]
	pub duplicate_rule_filter: DuplicateRuleFilter,
	#[serde(rename = "duplicateRuleMatchRules")]
	pub duplicate_rule_match_rules: Option<Vec<DuplicateRuleMatchRule>>,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "operationsOnInsert")]
	pub operations_on_insert: Option<Vec<String>>,
	#[serde(rename = "operationsOnUpdate")]
	pub operations_on_update: Option<Vec<String>>,
	#[serde(rename = "securityOption")]
	pub security_option: DupeSecurityOptionType,
	#[serde(rename = "sortOrder")]
	pub sort_order: i32,
}
