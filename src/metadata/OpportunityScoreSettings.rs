use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OpportunityScoreSettings  {
	#[serde(rename = "enableOpportunityScoring")]
	pub enable_opportunity_scoring: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
