use crate::metadata::DuplicateRuleFilterItem::DuplicateRuleFilterItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DuplicateRuleFilter  {
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: String,
	#[serde(rename = "duplicateRuleFilterItems")]
	pub duplicate_rule_filter_items: Option<Vec<DuplicateRuleFilterItem>>,
}
