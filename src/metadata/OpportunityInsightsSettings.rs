use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OpportunityInsightsSettings  {
	#[serde(rename = "enableOpportunityInsights")]
	pub enable_opportunity_insights: Option<bool>,
}
