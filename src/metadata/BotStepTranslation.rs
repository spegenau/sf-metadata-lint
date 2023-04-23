use crate::metadata::BotMessageTranslation::BotMessageTranslation;
use crate::metadata::BotStepType::BotStepType;
use crate::metadata::BotVariableOperationTranslation::BotVariableOperationTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotStepTranslation  {
	#[serde(rename = "botMessages")]
	pub bot_messages: Option<Vec<BotMessageTranslation>>,
	#[serde(rename = "botSteps")]
	pub bot_steps: Option<Vec<Box<BotStepTranslation>>>,
	#[serde(rename = "botVariableOperation")]
	pub bot_variable_operation: Option<BotVariableOperationTranslation>,
	#[serde(rename = "stepIdentifier")]
	pub step_identifier: String,
	#[serde(rename = "type")]
	pub _type: BotStepType,
}
