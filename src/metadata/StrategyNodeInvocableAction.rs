use crate::metadata::InvocableActionType::InvocableActionType;
use crate::metadata::StrategyNodeInvocableActionArg::StrategyNodeInvocableActionArg;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyNodeInvocableAction  {
	#[serde(rename = "action")]
	pub action: String,
	#[serde(rename = "argument")]
	pub argument: Option<Vec<StrategyNodeInvocableActionArg>>,
	#[serde(rename = "isGenerator")]
	pub is_generator: bool,
	#[serde(rename = "type")]
	pub _type: InvocableActionType,
}
