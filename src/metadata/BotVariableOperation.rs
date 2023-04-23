use crate::metadata::BotInvocation::BotInvocation;
use crate::metadata::BotMessage::BotMessage;
use crate::metadata::BotNavigation::BotNavigation;
use crate::metadata::BotQuickReplyOption::BotQuickReplyOption;
use crate::metadata::BotQuickReplyType::BotQuickReplyType;
use crate::metadata::BotVariableOperand::BotVariableOperand;
use crate::metadata::BotVariableOperationType::BotVariableOperationType;
use crate::metadata::BotWidgetType::BotWidgetType;
use crate::metadata::ConversationVariableType::ConversationVariableType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotVariableOperation  {
	#[serde(rename = "askCollectIfSet")]
	pub ask_collect_if_set: Option<bool>,
	#[serde(rename = "autoSelectIfSingleChoice")]
	pub auto_select_if_single_choice: Option<bool>,
	#[serde(rename = "botInvocation")]
	pub bot_invocation: Option<BotInvocation>,
	#[serde(rename = "botMessages")]
	pub bot_messages: Option<Vec<BotMessage>>,
	#[serde(rename = "botQuickReplyOptions")]
	pub bot_quick_reply_options: Option<Vec<BotQuickReplyOption>>,
	#[serde(rename = "botVariableOperands")]
	pub bot_variable_operands: Option<Vec<BotVariableOperand>>,
	#[serde(rename = "invalidInputBotNavigation")]
	pub invalid_input_bot_navigation: Option<BotNavigation>,
	#[serde(rename = "optionalCollect")]
	pub optional_collect: Option<bool>,
	#[serde(rename = "quickReplyOptionTemplate")]
	pub quick_reply_option_template: Option<String>,
	#[serde(rename = "quickReplyType")]
	pub quick_reply_type: Option<BotQuickReplyType>,
	#[serde(rename = "quickReplyWidgetType")]
	pub quick_reply_widget_type: Option<BotWidgetType>,
	#[serde(rename = "retryMessages")]
	pub retry_messages: Option<Vec<BotMessage>>,
	#[serde(rename = "sourceVariableName")]
	pub source_variable_name: Option<String>,
	#[serde(rename = "sourceVariableType")]
	pub source_variable_type: Option<ConversationVariableType>,
	#[serde(rename = "successMessages")]
	pub success_messages: Option<Vec<BotMessage>>,
	#[serde(rename = "type")]
	pub _type: BotVariableOperationType,
	#[serde(rename = "variableOperationIdentifier")]
	pub variable_operation_identifier: Option<String>,
}
