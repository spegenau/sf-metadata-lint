use crate::metadata::InvocableActionType::InvocableActionType;
use crate::metadata::StrategyActionArg::StrategyActionArg;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyAction  {
	#[serde(rename = "action")]
	pub action: String,
	#[serde(rename = "argument")]
	pub argument: Option<Vec<StrategyActionArg>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "type")]
	pub _type: InvocableActionType,
}
