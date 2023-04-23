use crate::metadata::BotStepConditionOperatorType::BotStepConditionOperatorType;
use crate::metadata::ConversationVariableType::ConversationVariableType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotStepCondition  {
	#[serde(rename = "leftOperandName")]
	pub left_operand_name: String,
	#[serde(rename = "leftOperandType")]
	pub left_operand_type: ConversationVariableType,
	#[serde(rename = "operatorType")]
	pub operator_type: BotStepConditionOperatorType,
	#[serde(rename = "rightOperandValue")]
	pub right_operand_value: Option<String>,
}
