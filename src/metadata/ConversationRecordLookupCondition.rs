use crate::metadata::ConversationVariableType::ConversationVariableType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationRecordLookupCondition  {
	#[serde(rename = "leftOperand")]
	pub left_operand: String,
	#[serde(rename = "operatorType")]
	pub operator_type: String,
	#[serde(rename = "rightOperandName")]
	pub right_operand_name: Option<String>,
	#[serde(rename = "rightOperandType")]
	pub right_operand_type: Option<ConversationVariableType>,
	#[serde(rename = "rightOperandValue")]
	pub right_operand_value: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: i32,
}
