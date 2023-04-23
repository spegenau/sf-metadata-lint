use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeAnswerSettings  {
	#[serde(rename = "assignTo")]
	pub assign_to: Option<String>,
	#[serde(rename = "defaultArticleType")]
	pub default_article_type: Option<String>,
	#[serde(rename = "enableArticleCreation")]
	pub enable_article_creation: Option<bool>,
}
