use crate::metadata::ModeratedEntityField::ModeratedEntityField;
use crate::metadata::ModerationRuleAction::ModerationRuleAction;
use crate::metadata::ModerationRuleType::ModerationRuleType;
use crate::metadata::RateLimitTimePeriod::RateLimitTimePeriod;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ModerationRule  {
	#[serde(rename = "action")]
	pub action: ModerationRuleAction,
	#[serde(rename = "actionLimit")]
	pub action_limit: Option<i32>,
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "entitiesAndFields")]
	pub entities_and_fields: Option<Vec<ModeratedEntityField>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "notifyLimit")]
	pub notify_limit: Option<i32>,
	#[serde(rename = "timePeriod")]
	pub time_period: Option<RateLimitTimePeriod>,
	#[serde(rename = "type")]
	pub _type: Option<ModerationRuleType>,
	#[serde(rename = "userCriteria")]
	pub user_criteria: Option<Vec<String>>,
	#[serde(rename = "userMessage")]
	pub user_message: Option<String>,
}
