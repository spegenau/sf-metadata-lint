use crate::metadata::BotDialog::BotDialog;
use crate::metadata::BotDialogGroup::BotDialogGroup;
use crate::metadata::ConversationContextVariable::ConversationContextVariable;
use crate::metadata::ConversationDefinitionGoal::ConversationDefinitionGoal;
use crate::metadata::ConversationSystemDialog::ConversationSystemDialog;
use crate::metadata::ConversationVariable::ConversationVariable;
use crate::metadata::LocalMlDomain::LocalMlDomain;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotTemplate  {
	#[serde(rename = "botDialogGroups")]
	pub bot_dialog_groups: Option<Vec<BotDialogGroup>>,
	#[serde(rename = "botDialogs")]
	pub bot_dialogs: Option<Vec<BotDialog>>,
	#[serde(rename = "contextVariables")]
	pub context_variables: Option<Vec<ConversationContextVariable>>,
	#[serde(rename = "conversationGoals")]
	pub conversation_goals: Option<Vec<ConversationDefinitionGoal>>,
	#[serde(rename = "conversationLanguages")]
	pub conversation_languages: String,
	#[serde(rename = "conversationSystemDialogs")]
	pub conversation_system_dialogs: Option<Vec<ConversationSystemDialog>>,
	#[serde(rename = "conversationVariables")]
	pub conversation_variables: Option<Vec<ConversationVariable>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "entryDialog")]
	pub entry_dialog: Option<String>,
	#[serde(rename = "icon")]
	pub icon: Option<String>,
	#[serde(rename = "mainMenuDialog")]
	pub main_menu_dialog: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "mlDomain")]
	pub ml_domain: Option<LocalMlDomain>,
	#[serde(rename = "richContentEnabled")]
	pub rich_content_enabled: Option<bool>,
}
