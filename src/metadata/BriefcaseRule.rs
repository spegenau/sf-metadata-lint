use crate::metadata::BriefcaseRuleFilter::BriefcaseRuleFilter;
use crate::metadata::BriefcaseRuleRelationshipType::BriefcaseRuleRelationshipType;
use crate::metadata::FilterScope::FilterScope;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BriefcaseRule  {
	#[serde(rename = "briefcaseRuleFilters")]
	pub briefcase_rule_filters: Option<Vec<BriefcaseRuleFilter>>,
	#[serde(rename = "filterLogic")]
	pub filter_logic: Option<String>,
	#[serde(rename = "isAscendingOrder")]
	pub is_ascending_order: Option<bool>,
	#[serde(rename = "orderBy")]
	pub order_by: Option<String>,
	#[serde(rename = "queryScope")]
	pub query_scope: Option<FilterScope>,
	#[serde(rename = "recordLimit")]
	pub record_limit: Option<i32>,
	#[serde(rename = "relatedRules")]
	pub related_rules: Option<Vec<Box<BriefcaseRule>>>,
	#[serde(rename = "relationshipField")]
	pub relationship_field: Option<String>,
	#[serde(rename = "relationshipType")]
	pub relationship_type: Option<BriefcaseRuleRelationshipType>,
	#[serde(rename = "targetEntity")]
	pub target_entity: String,
}
