use crate::metadata::BotVersion::BotVersion;
use crate::metadata::ConversationContextVariable::ConversationContextVariable;
use crate::metadata::ConversationDefinitionChannelProvider::ConversationDefinitionChannelProvider;
use crate::metadata::LocalMlDomain::LocalMlDomain;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Bot  {
	#[serde(rename = "botMlDomain")]
	pub bot_ml_domain: Option<LocalMlDomain>,
	#[serde(rename = "botUser")]
	pub bot_user: Option<String>,
	#[serde(rename = "botVersions")]
	pub bot_versions: Option<Vec<BotVersion>>,
	#[serde(rename = "contextVariables")]
	pub context_variables: Option<Vec<ConversationContextVariable>>,
	#[serde(rename = "conversationChannelProviders")]
	pub conversation_channel_providers: Option<Vec<ConversationDefinitionChannelProvider>>,
	#[serde(rename = "defaultOutboundFlow")]
	pub default_outbound_flow: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "logPrivateConversationData")]
	pub log_private_conversation_data: Option<bool>,
	#[serde(rename = "richContentEnabled")]
	pub rich_content_enabled: Option<bool>,
}
