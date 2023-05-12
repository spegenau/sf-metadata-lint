use crate::metadata::BotDialog::BotDialog;
use crate::metadata::ConvDefBlockVersionStatus::ConvDefBlockVersionStatus;
use crate::metadata::ConversationDefinitionGoal::ConversationDefinitionGoal;
use crate::metadata::ConversationVariable::ConversationVariable;
use crate::metadata::LocalMlDomain::LocalMlDomain;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotBlockVersion  {
	#[serde(rename = "botDialogs")]
	pub bot_dialogs: Option<Vec<BotDialog>>,
	#[serde(rename = "conversationGoals")]
	pub conversation_goals: Option<Vec<ConversationDefinitionGoal>>,
	#[serde(rename = "conversationLanguages")]
	pub conversation_languages: String,
	#[serde(rename = "conversationVariables")]
	pub conversation_variables: Option<Vec<ConversationVariable>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "mlDomain")]
	pub ml_domain: LocalMlDomain,
	#[serde(rename = "status")]
	pub status: ConvDefBlockVersionStatus,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
