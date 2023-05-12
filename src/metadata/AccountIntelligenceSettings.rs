use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AccountIntelligenceSettings  {
	#[serde(rename = "enableAccountLogos")]
	pub enable_account_logos: Option<bool>,
	#[serde(rename = "enableAutomatedAccountFields")]
	pub enable_automated_account_fields: Option<bool>,
	#[serde(rename = "enableNewsStories")]
	pub enable_news_stories: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
