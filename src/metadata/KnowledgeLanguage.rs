use crate::metadata::KnowledgeLanguageLookupValueType::KnowledgeLanguageLookupValueType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeLanguage  {
	#[serde(rename = "active")]
	pub active: Option<bool>,
	#[serde(rename = "defaultAssignee")]
	pub default_assignee: Option<String>,
	#[serde(rename = "defaultAssigneeType")]
	pub default_assignee_type: Option<KnowledgeLanguageLookupValueType>,
	#[serde(rename = "defaultReviewer")]
	pub default_reviewer: Option<String>,
	#[serde(rename = "defaultReviewerType")]
	pub default_reviewer_type: Option<KnowledgeLanguageLookupValueType>,
	#[serde(rename = "name")]
	pub name: String,
}
