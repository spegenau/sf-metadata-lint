use crate::metadata::FlowCondition::FlowCondition;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowVisibilityRule  {
	#[serde(rename = "conditionLogic")]
	pub condition_logic: Option<String>,
	#[serde(rename = "conditions")]
	pub conditions: Option<Vec<FlowCondition>>,
}
