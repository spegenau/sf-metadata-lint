use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyNodeInvocableActionArg  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "value")]
	pub value: String,
}
