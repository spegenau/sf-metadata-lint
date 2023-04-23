use crate::metadata::ConversationDefinitionNlpProviderType::ConversationDefinitionNlpProviderType;
use crate::metadata::Language::Language;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationDefinitionNlpProvider  {
	#[serde(rename = "language")]
	pub language: Option<Language>,
	#[serde(rename = "nlpProviderName")]
	pub nlp_provider_name: Option<String>,
	#[serde(rename = "nlpProviderType")]
	pub nlp_provider_type: ConversationDefinitionNlpProviderType,
}
