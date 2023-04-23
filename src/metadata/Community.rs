use crate::metadata::ReputationLevels::ReputationLevels;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Community  {
	#[serde(rename = "active")]
	pub active: Option<bool>,
	#[serde(rename = "chatterAnswersFacebookSsoUrl")]
	pub chatter_answers_facebook_sso_url: Option<String>,
	#[serde(rename = "communityFeedPage")]
	pub community_feed_page: Option<String>,
	#[serde(rename = "dataCategoryName")]
	pub data_category_name: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "emailFooterDocument")]
	pub email_footer_document: Option<String>,
	#[serde(rename = "emailHeaderDocument")]
	pub email_header_document: Option<String>,
	#[serde(rename = "emailNotificationUrl")]
	pub email_notification_url: Option<String>,
	#[serde(rename = "enableChatterAnswers")]
	pub enable_chatter_answers: Option<bool>,
	#[serde(rename = "enablePrivateQuestions")]
	pub enable_private_questions: Option<bool>,
	#[serde(rename = "expertsGroup")]
	pub experts_group: Option<String>,
	#[serde(rename = "portal")]
	pub portal: Option<String>,
	#[serde(rename = "reputationLevels")]
	pub reputation_levels: Option<ReputationLevels>,
	#[serde(rename = "showInPortal")]
	pub show_in_portal: Option<bool>,
	#[serde(rename = "site")]
	pub site: Option<String>,
}
