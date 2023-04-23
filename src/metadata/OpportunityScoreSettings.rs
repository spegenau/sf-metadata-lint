use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OpportunityScoreSettings  {
	#[serde(rename = "enableOpportunityScoring")]
	pub enable_opportunity_scoring: Option<bool>,
}
