use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AccountInsightsSettings  {
	#[serde(rename = "enableAccountInsights")]
	pub enable_account_insights: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
