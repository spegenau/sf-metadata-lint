use crate::metadata::BotDialog::BotDialog;
use crate::metadata::BotDialogGroup::BotDialogGroup;
use crate::metadata::ConversationDefinitionGoal::ConversationDefinitionGoal;
use crate::metadata::ConversationDefinitionNlpProvider::ConversationDefinitionNlpProvider;
use crate::metadata::ConversationSystemDialog::ConversationSystemDialog;
use crate::metadata::ConversationVariable::ConversationVariable;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotVersion  {
	#[serde(rename = "botDialogGroups")]
	pub bot_dialog_groups: Option<Vec<BotDialogGroup>>,
	#[serde(rename = "botDialogs")]
	pub bot_dialogs: Option<Vec<BotDialog>>,
	#[serde(rename = "conversationGoals")]
	pub conversation_goals: Option<Vec<ConversationDefinitionGoal>>,
	#[serde(rename = "conversationSystemDialogs")]
	pub conversation_system_dialogs: Option<Vec<ConversationSystemDialog>>,
	#[serde(rename = "conversationVariables")]
	pub conversation_variables: Option<Vec<ConversationVariable>>,
	#[serde(rename = "entryDialog")]
	pub entry_dialog: String,
	#[serde(rename = "mainMenuDialog")]
	pub main_menu_dialog: String,
	#[serde(rename = "nlpProviders")]
	pub nlp_providers: Option<Vec<ConversationDefinitionNlpProvider>>,
	#[serde(rename = "responseDelayMilliseconds")]
	pub response_delay_milliseconds: Option<i32>,
}
