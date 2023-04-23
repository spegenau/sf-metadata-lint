use crate::metadata::MatchingRuleItem::MatchingRuleItem;
use crate::metadata::MatchingRuleStatus::MatchingRuleStatus;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MatchingRule  {
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "matchingRuleItems")]
	pub matching_rule_items: Option<Vec<MatchingRuleItem>>,
	#[serde(rename = "ruleStatus")]
	pub rule_status: MatchingRuleStatus,
}
