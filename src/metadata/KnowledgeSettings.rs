use crate::metadata::KnowledgeAnswerSettings::KnowledgeAnswerSettings;
use crate::metadata::KnowledgeCaseSettings::KnowledgeCaseSettings;
use crate::metadata::KnowledgeLanguageSettings::KnowledgeLanguageSettings;
use crate::metadata::KnowledgeSuggestedArticlesSettings::KnowledgeSuggestedArticlesSettings;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeSettings  {
	#[serde(rename = "answers")]
	pub answers: Option<KnowledgeAnswerSettings>,
	#[serde(rename = "cases")]
	pub cases: Option<KnowledgeCaseSettings>,
	#[serde(rename = "defaultLanguage")]
	pub default_language: Option<String>,
	#[serde(rename = "enableChatterQuestionKBDeflection")]
	pub enable_chatter_question_kb_deflection: Option<bool>,
	#[serde(rename = "enableCreateEditOnArticlesTab")]
	pub enable_create_edit_on_articles_tab: Option<bool>,
	#[serde(rename = "enableExternalMediaContent")]
	pub enable_external_media_content: Option<bool>,
	#[serde(rename = "enableKbStandardSharing")]
	pub enable_kb_standard_sharing: Option<bool>,
	#[serde(rename = "enableKnowledge")]
	pub enable_knowledge: Option<bool>,
	#[serde(rename = "enableKnowledgeAgentContribution")]
	pub enable_knowledge_agent_contribution: Option<bool>,
	#[serde(rename = "enableKnowledgeAnswersPromotion")]
	pub enable_knowledge_answers_promotion: Option<bool>,
	#[serde(rename = "enableKnowledgeArticleTextHighlights")]
	pub enable_knowledge_article_text_highlights: Option<bool>,
	#[serde(rename = "enableKnowledgeCaseRL")]
	pub enable_knowledge_case_rl: Option<bool>,
	#[serde(rename = "enableKnowledgeKeywordAutoComplete")]
	pub enable_knowledge_keyword_auto_complete: Option<bool>,
	#[serde(rename = "enableKnowledgeTitleAutoComplete")]
	pub enable_knowledge_title_auto_complete: Option<bool>,
	#[serde(rename = "enableLightningKbAutoLoadRichTextField")]
	pub enable_lightning_kb_auto_load_rich_text_field: Option<bool>,
	#[serde(rename = "enableLightningKnowledge")]
	pub enable_lightning_knowledge: Option<bool>,
	#[serde(rename = "languages")]
	pub languages: Option<KnowledgeLanguageSettings>,
	#[serde(rename = "showArticleSummariesCustomerPortal")]
	pub show_article_summaries_customer_portal: Option<bool>,
	#[serde(rename = "showArticleSummariesInternalApp")]
	pub show_article_summaries_internal_app: Option<bool>,
	#[serde(rename = "showArticleSummariesPartnerPortal")]
	pub show_article_summaries_partner_portal: Option<bool>,
	#[serde(rename = "showValidationStatusField")]
	pub show_validation_status_field: Option<bool>,
	#[serde(rename = "suggestedArticles")]
	pub suggested_articles: Option<KnowledgeSuggestedArticlesSettings>,
	#[serde(rename = "votingEnabled")]
	pub voting_enabled: Option<bool>,
}
