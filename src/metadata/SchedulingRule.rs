use crate::metadata::SchedulingCategory::SchedulingCategory;
use crate::metadata::SchedulingRuleParameter::SchedulingRuleParameter;
use crate::metadata::SchedulingRuleType::SchedulingRuleType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SchedulingRule  {
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "schedulingCategory")]
	pub scheduling_category: SchedulingCategory,
	#[serde(rename = "schedulingRuleParameters")]
	pub scheduling_rule_parameters: Option<Vec<SchedulingRuleParameter>>,
	#[serde(rename = "schedulingRuleType")]
	pub scheduling_rule_type: SchedulingRuleType,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
