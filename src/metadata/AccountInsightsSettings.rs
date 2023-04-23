use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AccountInsightsSettings  {
	#[serde(rename = "enableAccountInsights")]
	pub enable_account_insights: Option<bool>,
}
