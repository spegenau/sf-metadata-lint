use crate::metadata::BotInvocation::BotInvocation;
use crate::metadata::BotMessage::BotMessage;
use crate::metadata::BotNavigation::BotNavigation;
use crate::metadata::BotStepCondition::BotStepCondition;
use crate::metadata::BotStepType::BotStepType;
use crate::metadata::BotVariableOperation::BotVariableOperation;
use crate::metadata::ConversationDefinitionRichMessage::ConversationDefinitionRichMessage;
use crate::metadata::ConversationDefinitionStepGoalMapping::ConversationDefinitionStepGoalMapping;
use crate::metadata::ConversationRecordLookup::ConversationRecordLookup;
use crate::metadata::ConversationSystemMessage::ConversationSystemMessage;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotStep  {
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: Option<String>,
	#[serde(rename = "botInvocation")]
	pub bot_invocation: Option<BotInvocation>,
	#[serde(rename = "botMessages")]
	pub bot_messages: Option<Vec<BotMessage>>,
	#[serde(rename = "botNavigation")]
	pub bot_navigation: Option<BotNavigation>,
	#[serde(rename = "botStepConditions")]
	pub bot_step_conditions: Option<Vec<BotStepCondition>>,
	#[serde(rename = "botSteps")]
	pub bot_steps: Option<Vec<Box<BotStep>>>,
	#[serde(rename = "botVariableOperation")]
	pub bot_variable_operation: Option<BotVariableOperation>,
	#[serde(rename = "conversationRecordLookup")]
	pub conversation_record_lookup: Option<ConversationRecordLookup>,
	#[serde(rename = "conversationStepGoalMappings")]
	pub conversation_step_goal_mappings: Option<Vec<ConversationDefinitionStepGoalMapping>>,
	#[serde(rename = "conversationSystemMessage")]
	pub conversation_system_message: Option<ConversationSystemMessage>,
	#[serde(rename = "messageDefinition")]
	pub message_definition: Option<ConversationDefinitionRichMessage>,
	#[serde(rename = "stepIdentifier")]
	pub step_identifier: Option<String>,
	#[serde(rename = "type")]
	pub _type: BotStepType,
}
