use crate::metadata::FlowCondition::FlowCondition;
use crate::metadata::FlowScreenRuleAction::FlowScreenRuleAction;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowScreenRule  {
	#[serde(rename = "conditionLogic")]
	pub condition_logic: Option<String>,
	#[serde(rename = "conditions")]
	pub conditions: Option<Vec<FlowCondition>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "ruleActions")]
	pub rule_actions: Option<Vec<FlowScreenRuleAction>>,
}
