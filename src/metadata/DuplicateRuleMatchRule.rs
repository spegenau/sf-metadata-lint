use crate::metadata::ObjectMapping::ObjectMapping;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DuplicateRuleMatchRule  {
	#[serde(rename = "matchRuleSObjectType")]
	pub match_rule_s_object_type: String,
	#[serde(rename = "matchingRule")]
	pub matching_rule: String,
	#[serde(rename = "objectMapping")]
	pub object_mapping: ObjectMapping,
}
